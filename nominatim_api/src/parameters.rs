use crate::{Coordinates, Location};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct SearchParameters {
    #[serde(flatten)]
    pub location: Location,
    pub format: String,
}

#[derive(Serialize)]
pub struct ReverseParameters {
    #[serde(flatten)]
    pub coordinates: Coordinates,
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
