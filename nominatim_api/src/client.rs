use crate::{
    parameters::{ReverseParameters, ReverseResponse, SearchParameters, SearchResponse},
    Coordinates, Error, Location,
};
use reqwest::Url;

pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    email: String,
    format: String,
    zoom: u8,
}

pub struct ClientBuilder {
    base_url: Option<Url>,
    email: Option<String>,
    format: String,
    zoom: u8,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            email: None,
            format: "geocodejson".into(),
            zoom: 10, // Zoom 10 means city
        }
    }
}

impl ClientBuilder {
    pub fn base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = Some(Url::parse(&base_url).expect("The provided base url is invalid"));
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn format(&mut self, format: String) -> &mut Self {
        self.format = format;
        self
    }

    pub fn zoom(&mut self, zoom: u8) -> &mut Self {
        self.zoom = zoom;
        self
    }

    pub fn build(&mut self) -> Client {
        let client_builder = reqwest::Client::new();
        Client {
            client: client_builder,
            base_url: self.base_url.clone().expect("Please provide a base url"),
            email: self.email.clone().expect("Please provide an email"),
            format: self.format.clone(),
            zoom: self.zoom,
        }
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn search(&self, location: Location) -> Result<Coordinates, Error> {
        let parameters = SearchParameters {
            location: location,
            email: self.email.clone(),
            format: self.format.clone(),
        };

        let request = self
            .client
            .get(self.base_url.join("/search")?)
            .query(&parameters)
            .build()?;
        let response = self.client.execute(request).await?;
        let search_response = response.json::<SearchResponse>().await?;

        if search_response.features.is_empty() {
            return Err(Error::Parsing(
                "The search response contained no features".into(),
            ));
        }

        let geometry_coordinates = &search_response
            .features
            .first()
            .unwrap()
            .geometry
            .coordinates;

        if geometry_coordinates.len() != 2 {
            return Err(Error::Parsing(format!(
                "The search response provided an invalid number of coordinates {}",
                geometry_coordinates.len()
            )));
        }

        Ok(Coordinates {
            lat: geometry_coordinates[1],
            lon: geometry_coordinates[0],
        })
    }

    pub async fn search_list(&self, locations: Vec<Location>) -> Result<Vec<Coordinates>, Error> {
        futures::future::join_all(locations.into_iter().map(|location| self.search(location)))
            .await
            .into_iter()
            .collect()
    }

    pub async fn reverse(&self, coordinates: Coordinates) -> Result<(String, String), Error> {
        let parameters = ReverseParameters {
            coordinates: coordinates,
            email: self.email.clone(),
            format: self.format.clone(),
            zoom: self.zoom,
        };

        let request = self
            .client
            .get(self.base_url.join("/reverse")?)
            .query(&parameters)
            .build()?;
        let response = self.client.execute(request).await?;

        let reverse_response = response.json::<ReverseResponse>().await?;

        if reverse_response.features.is_empty() {
            return Err(Error::Parsing(
                "The reverse response contained no features".into(),
            ));
        }

        let geocoding = &reverse_response
            .features
            .first()
            .unwrap()
            .properties
            .geocoding;

        if geocoding.contains_key("name") && geocoding.contains_key("country") {
            return Ok((
                geocoding["name"]
                    .as_str()
                    .ok_or(Error::Parsing(
                        "The reverse response returned an invalid name".into(),
                    ))?
                    .into(),
                geocoding["country"]
                    .as_str()
                    .ok_or(Error::Parsing(
                        "The reverse response returned an invalid name".into(),
                    ))?
                    .into(),
            ));
        }

        Err(Error::Parsing(
            "The reverse response did not contain the name or country".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    const BASE_URL: &'static str = "https://nominatim.openstreetmap.org/";
    const EMAIL: &'static str = "user@company.com";

    #[test]
    fn client_construction() {
        let client = Client::builder()
            .base_url(BASE_URL.into())
            .email(EMAIL.into())
            .build();

        assert_eq!(BASE_URL, client.base_url.as_str());
        assert_eq!(EMAIL, client.email);
    }

    #[tokio::test]
    async fn search_test() {
        let client = Client::builder()
            .base_url(BASE_URL.into())
            .email(EMAIL.into())
            .build();

        let location = Location {
            street: "Spanien 1".to_string(),
            county: None,
            state: None,
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: None,
        };

        let search_result = client.search(location).await;
        println!("{:?}", search_result);
        assert!(search_result.is_ok());

        let expected_coordinates = Coordinates {
            lon: 10.210985,
            lat: 56.1518,
        };
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }

    #[tokio::test]
    async fn search_list_test() {
        let client = Client::builder()
            .base_url(BASE_URL.into())
            .email(EMAIL.into())
            .build();

        let locations = vec![
            Location {
                street: "Spanien 1".to_string(),
                city: "Aarhus".to_string(),
                country: "Denmark".to_string(),
                ..Default::default()
            },
            Location {
                street: "Spanien 3".to_string(),
                city: "Aarhus".to_string(),
                country: "Denmark".to_string(),
                ..Default::default()
            },
            Location {
                street: "Spanien 5".to_string(),
                city: "Aarhus".to_string(),
                country: "Denmark".to_string(),
                ..Default::default()
            },
        ];

        let search_result = client.search_list(locations).await;
        assert!(search_result.is_ok());

        let expected_coordinates: Vec<Coordinates> = vec![
            Coordinates {
                lat: 56.1518,
                lon: 10.210985,
            },
            Coordinates {
                lat: 56.151714,
                lon: 10.210944,
            },
            Coordinates {
                lat: 56.149483,
                lon: 10.20983,
            },
        ];
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }

    #[tokio::test]
    async fn reverse_test() {
        let client = Client::builder()
            .base_url(BASE_URL.into())
            .email(EMAIL.into())
            .build();

        let coordinates = Coordinates {
            lon: 10.210985,
            lat: 56.1518,
        };

        let reverse_result = client.reverse(coordinates).await;
        assert!(reverse_result.is_ok());

        let expected_response = ("Aarhus".to_string(), "Danmark".to_string());
        assert_eq!(reverse_result.unwrap(), expected_response);
    }
}
