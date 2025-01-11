mod requests;
mod responses;

fn main() {
    println!("Serialization:");
    requests::serialization();

    println!("{}", "-".repeat(25));

    println!("\nDeserialization:");
    requests::deserialization();

    println!("{}", "-".repeat(25));

    println!("Serialization:");
    responses::serialization();

    println!("{}", "-".repeat(25));

    println!("\nDeserialization:");
    responses::deserialization();
}
