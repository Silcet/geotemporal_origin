use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Coordinate;

// TODO: Turn into builder pattern
#[derive(Serialize)]
pub struct SearchParameters {
    pub street: String,
    pub city: String,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postalcode: Option<u16>,
    pub email: String,
    pub format: String,
}

#[derive(Serialize)]
pub struct ReverseParameters {
    pub lat: f32,
    pub lon: f32,
    pub email: String,
    pub format: String,
    pub zoom: u8,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    #[serde(skip_deserializing)]
    r#type: String,
    pub coordinates: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub geocoding: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct Features {
    #[serde(skip_deserializing)]
    r#type: String,
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(skip_deserializing)]
    r#type: String,
    #[serde(skip_deserializing)]
    geocoding: HashMap<String, String>,

    pub features: Vec<Features>,
}

#[derive(Deserialize, Debug)]
pub struct ReverseResponse {
    #[serde(skip_deserializing)]
    r#type: String,
    #[serde(skip_deserializing)]
    geocoding: HashMap<String, String>,

    pub features: Vec<Features>,
}

impl ReverseParameters {
    pub fn new(coordinates: Coordinate, email: String, format: String, zoom: u8) -> Self {
        Self {
            lat: coordinates.lat,
            lon: coordinates.lon,
            email,
            format,
            zoom,
        }
    }
}
