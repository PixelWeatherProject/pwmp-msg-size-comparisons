use pwmp_msg::{response::Response, settings::NodeSettings, version::Version, Message};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

fn build_responses() -> Vec<Message> {
    let pong = Message::new_response(Response::Pong, 887_412);
    let ok = Message::new_response(Response::Ok, 887_412);
    let reject = Message::new_response(Response::Reject, 887_412);
    let invalid_request = Message::new_response(Response::InvalidRequest, 887_412);
    let rate_limit_exceeded = Message::new_response(Response::RateLimitExceeded, 887_412);
    let internal_server_error = Message::new_response(Response::InternalServerError, 887_412);
    let stalling = Message::new_response(Response::Stalling, 887_412);
    let fw_up_to_date = Message::new_response(Response::FirmwareUpToDate, 887_412);
    let update_available =
        Message::new_response(Response::UpdateAvailable(Version::new(1, 2, 3)), 887_412);
    let update_part = Message::new_response(
        Response::UpdatePart(b"A".to_vec().into_boxed_slice()),
        887_412,
    );
    let update_end = Message::new_response(Response::UpdateEnd, 887_412);
    let settings = Message::new_response(
        Response::Settings(Some(NodeSettings {
            battery_ignore: false,
            ota: true,
            sleep_time: 60,
            sbop: true,
            mute_notifications: false,
        })),
        887_412,
    );

    let requests = vec![
        pong,
        ok,
        reject,
        invalid_request,
        rate_limit_exceeded,
        internal_server_error,
        stalling,
        fw_up_to_date,
        update_available,
        update_part,
        update_end,
        settings,
    ];

    requests
}

pub fn serialization() -> Vec<(usize, Duration)> {
    let requests = build_responses();

    let mut avg_bincode_size = 0;
    let mut avg_bincode_time = Duration::from_secs(0);

    let mut avg_msgpack_size = 0;
    let mut avg_msgpack_time = Duration::from_secs(0);

    let mut avg_postcard_size = 0;
    let mut avg_postcard_time = Duration::from_secs(0);

    for msg in &requests {
        let (bincode_size, bincode_time) = benchmark_serialize(msg, |val| {
            bincode::serde::encode_to_vec(val, bincode::config::legacy())
        });
        avg_bincode_size += bincode_size;
        avg_bincode_time += bincode_time;

        let (msgpack_size, msgpack_time) = benchmark_serialize(msg, rmp_serde::to_vec);
        avg_msgpack_size += msgpack_size;
        avg_msgpack_time += msgpack_time;

        let (postcard_size, postcard_time) = benchmark_serialize(msg, postcard::to_allocvec);
        avg_postcard_size += postcard_size;
        avg_postcard_time += postcard_time;

        println!("{}", "-".repeat(20));
        println!("Bincode: {bincode_size} bytes, took {bincode_time:?}");
        println!("Msgpack: {msgpack_size} bytes, took {msgpack_time:?}");
        println!("Postcard: {postcard_size} bytes, took {postcard_time:?}");
        println!("{}", "-".repeat(20));
    }

    avg_bincode_size /= requests.len();
    avg_bincode_time /= requests.len() as u32;
    avg_msgpack_size /= requests.len();
    avg_msgpack_time /= requests.len() as u32;
    avg_postcard_size /= requests.len();
    avg_postcard_time /= requests.len() as u32;

    println!("\nSUMMARY (responses):");
    println!("{}", "-".repeat(20));
    println!("Avg Bincode: {avg_bincode_size} bytes, time: {avg_bincode_time:?}");
    println!("Avg Msgpack: {avg_msgpack_size} bytes, time: {avg_msgpack_time:?}");
    println!("Avg Postcard: {avg_postcard_size} bytes, time: {avg_postcard_time:?}");
    println!("{}", "-".repeat(20));

    vec![
        (avg_bincode_size, avg_bincode_time),
        (avg_msgpack_size, avg_msgpack_time),
        (avg_postcard_size, avg_postcard_time),
    ]
}

pub fn deserialization() -> Vec<Duration> {
    let requests = build_responses();

    let mut avg_bincode_time = Duration::from_secs(0);
    let mut avg_msgpack_time = Duration::from_secs(0);
    let mut avg_postcard_time = Duration::from_secs(0);

    for msg in &requests {
        let raw = bincode::serde::encode_to_vec(msg, bincode::config::legacy()).unwrap();
        let bincode_time = benchmark_deserialize(&raw, |raw| {
            bincode::serde::decode_from_slice::<Message, _>(raw, bincode::config::legacy())
        });
        avg_bincode_time += bincode_time;

        let raw = rmp_serde::to_vec(&msg).unwrap();
        let msgpack_time = benchmark_deserialize(&raw, rmp_serde::from_slice::<Message>);
        avg_msgpack_time += msgpack_time;

        let raw = postcard::to_allocvec_cobs(&msg).unwrap();
        let postcard_time = benchmark_deserialize(&raw, postcard::from_bytes::<Message>);
        avg_postcard_time += postcard_time;

        println!("{}", "-".repeat(20));
        println!("Bincode: took {bincode_time:?}");
        println!("Msgpack: took {msgpack_time:?}");
        println!("Postcard: took {postcard_time:?}");
        println!("{}", "-".repeat(20));
    }

    avg_bincode_time /= requests.len() as u32;
    avg_msgpack_time /= requests.len() as u32;
    avg_postcard_time /= requests.len() as u32;

    println!("\nSUMMARY (responses):");
    println!("{}", "-".repeat(20));
    println!("Avg Bincode: time: {avg_bincode_time:?}");
    println!("Avg Msgpack: time: {avg_msgpack_time:?}");
    println!("Avg Postcard: time: {avg_postcard_time:?}");
    println!("{}", "-".repeat(20));

    vec![avg_bincode_time, avg_msgpack_time, avg_postcard_time]
}

fn benchmark_serialize<T, S, E>(what: &T, serializer: S) -> (usize, Duration)
where
    T: Serialize,
    S: FnOnce(&T) -> Result<Vec<u8>, E>,
    E: Debug,
{
    let start = Instant::now();
    let raw = serializer(what);
    let elapsed = start.elapsed();

    (raw.unwrap().len(), elapsed)
}

fn benchmark_deserialize<'a, T, D, E>(raw: &'a [u8], deserializer: D) -> Duration
where
    T: DeserializeOwned,
    D: FnOnce(&'a [u8]) -> Result<T, E>,
    E: Debug,
{
    let start = Instant::now();
    let _obj = deserializer(raw);

    start.elapsed()
}
