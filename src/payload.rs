use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

#[derive(Deserialize)]
pub struct Longurl {
    pub long_url: String,
}
