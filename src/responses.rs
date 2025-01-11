use pwmp_msg::{response::Response, settings::NodeSettings, version::Version, Message};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

fn build_responses() -> Vec<Message> {
    let pong = Message::Response(Response::Pong);
    let ok = Message::Response(Response::Ok);
    let reject = Message::Response(Response::Reject);
    let fw_up_to_date = Message::Response(Response::FirmwareUpToDate);
    let update_available = Message::Response(Response::UpdateAvailable(Version::new(1, 2, 3)));
    let update_part = Message::Response(Response::UpdatePart(
        b"ABCDEFGHIJKLMNOPQRSTUVXYZ0123456"
            .repeat(1024)
            .into_boxed_slice(),
    ));
    let update_end = Message::Response(Response::UpdateEnd);
    let settings = Message::Response(Response::Settings(NodeSettings {
        battery_ignore: false,
        ota: true,
        sleep_time: 60,
        sbop: true,
        mute_notifications: false,
    }));

    let requests = vec![
        pong,
        ok,
        reject,
        fw_up_to_date,
        update_available,
        update_part,
        update_end,
        settings,
    ];

    requests
}

pub fn serialization() {
    let requests = build_responses();

    let mut avg_bincode_size = 0;
    let mut avg_bincode_time = Duration::from_secs(0);

    let mut avg_msgpack_size = 0;
    let mut avg_msgpack_time = Duration::from_secs(0);

    let mut avg_postcard_size = 0;
    let mut avg_postcard_time = Duration::from_secs(0);

    for msg in &requests {
        let (bincode_size, bincode_time) = benchmark_serialize(msg, bincode::serialize);
        avg_bincode_size += bincode_size;
        avg_bincode_time += bincode_time;

        let (msgpack_size, msgpack_time) = benchmark_serialize(msg, rmp_serde::to_vec);
        avg_msgpack_size += msgpack_size;
        avg_msgpack_time += msgpack_time;

        let (postcard_size, postcard_time) = benchmark_serialize(msg, postcard::to_allocvec);
        avg_postcard_size += postcard_size;
        avg_postcard_time += postcard_time;

        println!("{}", "-".repeat(20));
        println!(
            "Message: {}",
            format!("{msg:?}").chars().take(30).collect::<String>()
        );
        println!("Bincode: {} bytes, took {:?}", bincode_size, bincode_time);
        println!("Msgpack: {} bytes, took {:?}", msgpack_size, msgpack_time);
        println!(
            "Postcard: {} bytes, took {:?}",
            postcard_size, postcard_time
        );
        println!("{}", "-".repeat(20));
    }

    avg_bincode_size /= requests.len();
    avg_bincode_time /= requests.len() as u32;
    avg_msgpack_size /= requests.len();
    avg_msgpack_time /= requests.len() as u32;
    avg_postcard_size /= requests.len();
    avg_postcard_time /= requests.len() as u32;

    println!("\nSUMMARY:");
    println!("{}", "-".repeat(20));
    println!(
        "Avg Bincode: {} bytes, time: {:?}",
        avg_bincode_size, avg_bincode_time
    );
    println!(
        "Avg Msgpack: {} bytes, time: {:?}",
        avg_msgpack_size, avg_msgpack_time
    );
    println!(
        "Avg Postcard: {} bytes, time: {:?}",
        avg_postcard_size, avg_postcard_time
    );
    println!("{}", "-".repeat(20));
}

pub fn deserialization() {
    let requests = build_responses();

    let mut avg_bincode_time = Duration::from_secs(0);
    let mut avg_msgpack_time = Duration::from_secs(0);
    let mut avg_postcard_time = Duration::from_secs(0);

    for msg in &requests {
        let raw = bincode::serialize(msg).unwrap();
        let bincode_time = benchmark_deserialize(&raw, bincode::deserialize::<Message>);
        avg_bincode_time += bincode_time;

        let raw = rmp_serde::to_vec(&msg).unwrap();
        let msgpack_time = benchmark_deserialize(&raw, rmp_serde::from_slice::<Message>);
        avg_msgpack_time += msgpack_time;

        let raw = postcard::to_allocvec_cobs(&msg).unwrap();
        let postcard_time = benchmark_deserialize(&raw, postcard::from_bytes::<Message>);
        avg_postcard_time += postcard_time;

        println!("{}", "-".repeat(20));
        println!("Bincode: took {:?}", bincode_time);
        println!("Msgpack: took {:?}", msgpack_time);
        println!("Postcard: took {:?}", postcard_time);
        println!("{}", "-".repeat(20));
    }

    avg_bincode_time /= requests.len() as u32;
    avg_msgpack_time /= requests.len() as u32;
    avg_postcard_time /= requests.len() as u32;

    println!("\nSUMMARY:");
    println!("{}", "-".repeat(20));
    println!("Avg Bincode: time: {:?}", avg_bincode_time);
    println!("Avg Msgpack: time: {:?}", avg_msgpack_time);
    println!("Avg Postcard: time: {:?}", avg_postcard_time);
    println!("{}", "-".repeat(20));
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
