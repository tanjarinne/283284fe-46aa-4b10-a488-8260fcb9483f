use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;

#[macro_use]
extern crate rocket;

mod structures;
use structures::HealthResponse;

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(db::mount_aerospike_client())
        .mount("/", routes![healthcheck])
}

#[get("/health")]
pub async fn healthcheck(state: &State<db::AppState>) -> Result<Json<HealthResponse>, Status> {
    let response = HealthResponse {
        status: state.aerospike.is_connected().to_string(),
        message: "dummy message".to_string(),
    };
    Ok(Json(response))
}
