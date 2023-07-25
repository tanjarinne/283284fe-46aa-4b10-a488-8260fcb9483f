use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UrlRequest<'r> {
  pub url: &'r str,
}

#[derive(Serialize)]
pub struct Response {
  pub url        : String,
  pub shorten_url: String
}

#[derive(Serialize)]
pub struct HealthResponse {
  pub status: String,
  pub message: String,
}
