use thiserror::Error;

#[derive(Error, Debug)]
pub enum NominatimError {
    #[error("Error calling nominatim API")]
    API(#[from] reqwest::Error),
    #[error("Deserializing error: {0}")]
    Parsing(String),
    #[error("Error parsing url")]
    UrlParsing(#[from] url::ParseError),
    #[error("{0} is not a valid {1}")]
    InvalidCoordinate(f32, String),
    #[error("Invalid header")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

pub type Result<T> = std::result::Result<T, NominatimError>;
