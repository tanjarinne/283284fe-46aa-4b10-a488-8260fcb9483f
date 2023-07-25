use aerospike::{Client, ClientPolicy};
use rocket::serde::json::Json;
use rocket::http::Status;

#[macro_use]
extern crate rocket;

mod structures;
use structures::HealthResponse;

#[launch]
fn rocket() -> _ {
    let as_client = connect_to_aerospike();
    rocket::build()
        .mount("/", routes![healthcheck])
}

#[get("/health")]
pub async fn healthcheck() -> Result<Json<HealthResponse>, Status> {
    let response = HealthResponse {
        status: "dummy success".to_string(),
        message: "dummy message".to_string(),
    };
    Ok(Json(response))
}

fn connect_to_aerospike() -> Client {
    let hosts: String = std::env::var("AEROSPIKE_HOSTS")
        .unwrap_or(String::from("localhost:3000"));
    let policy = ClientPolicy::default();

    return Client::new(&policy, &hosts)
        .expect("Failed to connect to Aerospike");
}
