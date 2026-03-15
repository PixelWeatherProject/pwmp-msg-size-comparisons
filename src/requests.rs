use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

fn build_requests() -> Vec<Message> {
    let hello = Message::new_request(
        Request::Handshake {
            mac: Mac::new(1, 2, 3, 4, 5, 6),
        },
        8_716_978,
    );
    let post_results = Message::new_request(
        Request::PostResults {
            temperature: 20.44,
            humidity: 67,
            air_pressure: Some(u16::MAX),
        },
        8_716_978,
    );
    let post_stats = Message::new_request(
        Request::PostStats {
            battery: 4.20,
            wifi_ssid: "ABCDEFGHIJKLMNOPQRSTUVXYZ0123456".into(),
            wifi_rssi: -85,
        },
        8_716_978,
    );
    let send_notification = Message::new_request(
        Request::SendNotification("The battery is too low, activating sBOP.".into()),
        8_716_978,
    );
    let get_settings = Message::new_request(Request::GetSettings, 8_716_978);
    let update_check = Message::new_request(Request::UpdateCheck(Version::new(1, 0, 1)), 8_716_978);
    let next_update_chunk = Message::new_request(Request::NextUpdateChunk(1024), 8_716_978);
    let report_firmware_update =
        Message::new_request(Request::ReportFirmwareUpdate(false), 8_716_978);
    let bye = Message::new_request(Request::Bye, 8_716_978);

    let requests = vec![
        hello,
        post_results,
        post_stats,
        send_notification,
        get_settings,
        update_check,
        next_update_chunk,
        report_firmware_update,
        bye,
    ];

    requests
}

pub fn serialization() {
    let requests = build_requests();

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
        println!("Message: {msg:?}");
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
    let requests = build_requests();

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
