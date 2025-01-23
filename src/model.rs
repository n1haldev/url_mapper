use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UrlMap {
    pub short_url: String,
    pub long_url: String,
}
