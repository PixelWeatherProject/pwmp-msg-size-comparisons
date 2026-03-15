#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]

use std::time::Duration;

mod requests;
mod responses;

fn main() {
    println!("Serialization:");
    let req_ser_perf = requests::serialization();

    println!("{}", "-".repeat(25));

    println!("\nDeserialization:");
    let req_des_perf = requests::deserialization();

    println!("{}", "-".repeat(25));

    println!("Serialization:");
    let res_ser_perf = responses::serialization();

    println!("{}", "-".repeat(25));

    println!("\nDeserialization:");
    let res_des_perf = responses::deserialization();

    let (avg_bincode_size, avg_bincode_ser_time) =
        summarize_ser(&[req_ser_perf[0], res_ser_perf[0]]);
    let (avg_msgpack_size, avg_msgpack_ser_time) =
        summarize_ser(&[req_ser_perf[1], res_ser_perf[1]]);
    let (avg_postcard_size, avg_postcard_ser_time) =
        summarize_ser(&[req_ser_perf[2], res_ser_perf[2]]);

    let avg_bincode_des_time = summarize_des(&[req_des_perf[0], res_des_perf[0]]);
    let avg_msgpack_des_time = summarize_des(&[req_des_perf[1], res_des_perf[1]]);
    let avg_postcard_des_time = summarize_des(&[req_des_perf[2], res_des_perf[2]]);

    println!("{}", "-".repeat(40));
    println!("COMPLETE SUMMARY:");

    println!("Serialization:");
    println!("\tbincode: size: {avg_bincode_size}, time: {avg_bincode_ser_time:?}");
    println!("\tmsgpack: size: {avg_msgpack_size}, time: {avg_msgpack_ser_time:?}");
    println!("\tpostcard: size: {avg_postcard_size}, time: {avg_postcard_ser_time:?}");

    println!("Deserialization:");
    println!("\tbincode: time: {avg_bincode_des_time:?}");
    println!("\tmsgpack: time: {avg_msgpack_des_time:?}");
    println!("\tpostcard: time: {avg_postcard_des_time:?}");
}

fn summarize_des(results: &[Duration]) -> Duration {
    let sum = results.iter().sum::<Duration>().as_secs_f64();
    let len = f64::from(u32::try_from(results.len()).unwrap());

    Duration::from_secs_f64(sum / len)
}

fn summarize_ser(results: &[(usize, Duration)]) -> (usize, Duration) {
    let sizes: Vec<f64> = results
        .iter()
        .map(|e| f64::from(u32::try_from(e.0).unwrap()))
        .collect();
    let durations: Vec<f64> = results.iter().map(|e| e.1.as_secs_f64()).collect();

    let avg_size = sizes.iter().sum::<f64>() / f64::from(u32::try_from(sizes.len()).unwrap());
    let avg_dur =
        durations.iter().sum::<f64>() / f64::from(u32::try_from(durations.len()).unwrap());

    (avg_size as usize, Duration::from_secs_f64(avg_dur))
}
