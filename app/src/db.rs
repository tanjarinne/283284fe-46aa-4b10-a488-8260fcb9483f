use aerospike::{Client, ClientPolicy, WritePolicy, as_key, as_bin, ReadPolicy, Bins, Record};
use rocket::State;

pub struct AppState {
  pub aerospike: Client,
}

pub fn get_record(state: &State<AppState>, url_hash: String) -> Result<Record, String> {
    let key = as_key!("test", "urls", &url_hash.to_string());
    match state.aerospike.get(&ReadPolicy::default(), &key, Bins::from(["long_url"])) {
        Ok(record) => Ok(record),
        Err(err) => Err((&err).to_string()),
    }
}

pub fn put_record(state: &State<AppState>, long_url: String, url_hash: String) -> Result<(), String> {
  let policy = WritePolicy::default();
  let key = as_key!("test", "urls", url_hash.as_str());
  let bin = as_bin!("long_url", long_url.as_str());

  match state.aerospike.put(&policy, &key, &vec![&bin]) {
    Ok(()) => Ok(()),
    Err(err) => Err((&err).to_string()),
  }
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
