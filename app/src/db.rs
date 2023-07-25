use aerospike::{Client, ClientPolicy};

pub struct AppState {
  pub aerospike: Client,
}

pub fn mount_aerospike_client() -> AppState {
    let client = connect_to_aerospike();
    AppState { aerospike: client }
}

fn connect_to_aerospike() -> Client {
    let hosts: String = std::env::var("AEROSPIKE_HOSTS")
        .unwrap_or(String::from("localhost:3000"));
    let policy = ClientPolicy::default();

    return Client::new(&policy, &hosts)
        .expect("Failed to connect to Aerospike")
}
