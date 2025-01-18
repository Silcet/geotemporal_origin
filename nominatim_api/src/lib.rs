pub mod client;
pub mod parameters;

use reqwest;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error calling nominatim API")]
    API(#[from] reqwest::Error),
    #[error("Deserializing error: {0}")]
    Parsing(String),
    #[error("Error parsing url")]
    UrlParsing(#[from] url::ParseError),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Coordinates {
    lat: f32,
    lon: f32,
}

#[derive(Serialize, Default, Clone)]
pub struct Location {
    pub street: String,
    pub city: String,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postalcode: Option<u16>,
}
