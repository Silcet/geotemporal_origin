mod client;
mod search;

use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error calling nominatim API")]
    API(#[from] reqwest::Error),
    #[error("Error parsing API response")]
    Parsing(#[from] json::Error),
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    latitude: f32,
    longitude: f32,
}
