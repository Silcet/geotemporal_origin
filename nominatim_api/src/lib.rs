mod client;
mod parameters;

use reqwest;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error calling nominatim API")]
    API(#[from] reqwest::Error),
    #[error("Deserializing error: {0}")]
    Parsing(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Coordinate {
    lat: f32,
    lon: f32,
}
