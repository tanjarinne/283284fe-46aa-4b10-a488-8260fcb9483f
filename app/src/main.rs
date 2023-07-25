use aerospike::{Client, ClientPolicy};
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;

#[macro_use]
extern crate rocket;

mod structures;
use structures::HealthResponse;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(mount_aerospike_client())
        .mount("/", routes![healthcheck])
}

pub struct AppState {
    aerospike: Client,
}

fn mount_aerospike_client() -> AppState {
    let client = connect_to_aerospike();
    AppState { aerospike: client }
}

#[get("/health")]
pub async fn healthcheck(state: &State<AppState>) -> Result<Json<HealthResponse>, Status> {
    let response = HealthResponse {
        status: state.aerospike.is_connected().to_string(),
        message: "dummy message".to_string(),
    };
    Ok(Json(response))
}

fn connect_to_aerospike() -> Client {
    let hosts: String = std::env::var("AEROSPIKE_HOSTS")
        .unwrap_or(String::from("localhost:3000"));
    let policy = ClientPolicy::default();

    return Client::new(&policy, &hosts)
        .expect("Failed to connect to Aerospike")
}
