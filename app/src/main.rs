#[macro_use]

use aerospike::{Client, ClientPolicy};

fn main() {
    let as_client = connect_to_aerospike();
    println!("Hello, world!");
}

fn connect_to_aerospike() -> Client {
    let hosts: String = std::env::var("AEROSPIKE_HOSTS")
        .unwrap_or(String::from("localhost:3000"));
    let policy = ClientPolicy::default();

    return Client::new(&policy, &hosts)
        .expect("Failed to connect to Aerospike");
}
