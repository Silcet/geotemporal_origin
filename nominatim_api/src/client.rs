use crate::{
    parameters::{ReverseParameters, ReverseResponse, SearchParameters, SearchResponse},
    Coordinates, Location, NominatimError, Result,
};
use reqwest::Url;

pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    referer: String,
    format: String,
    zoom: u8,
}

pub struct ClientBuilder {
    base_url: Option<Url>,
    referer: String,
    format: String,
    zoom: u8,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            referer: "http://silcet.github.io/geotemporal_origin/".into(),
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

    pub fn referer(&mut self, referer: String) -> &mut Self {
        self.referer = referer;
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
            referer: self.referer.clone(),
            format: self.format.clone(),
            zoom: self.zoom,
        }
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn search(&self, location: Location) -> Result<Coordinates> {
        let parameters = SearchParameters {
            location: location,
            format: self.format.clone(),
        };

        let request = self
            .client
            .get(self.base_url.join("/search")?)
            .header(
                reqwest::header::REFERER,
                reqwest::header::HeaderValue::from_str(&self.referer)?,
            )
            .query(&parameters)
            .build()?;
        let response = self.client.execute(request).await?;
        let search_response = response.json::<SearchResponse>().await?;

        if search_response.features.is_empty() {
            return Err(NominatimError::Parsing(
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
            return Err(NominatimError::Parsing(format!(
                "The search response provided an invalid number of coordinates {}",
                geometry_coordinates.len()
            )));
        }

        // The api provides first longitude then latitude
        Coordinates::try_new(geometry_coordinates[1], geometry_coordinates[0])
    }

    pub async fn search_list(&self, locations: Vec<Location>) -> Result<Vec<Coordinates>> {
        futures::future::join_all(locations.into_iter().map(|location| self.search(location)))
            .await
            .into_iter()
            .collect()
    }

    pub async fn reverse(&self, coordinates: Coordinates) -> Result<(String, String)> {
        let parameters = ReverseParameters {
            coordinates: coordinates,
            format: self.format.clone(),
            zoom: self.zoom,
        };

        let request = self
            .client
            .get(self.base_url.join("/reverse")?)
            .header(
                reqwest::header::REFERER,
                reqwest::header::HeaderValue::from_str(&self.referer)?,
            )
            .query(&parameters)
            .build()?;
        let response = self.client.execute(request).await?;

        let reverse_response = response.json::<ReverseResponse>().await?;

        if reverse_response.features.is_empty() {
            return Err(NominatimError::Parsing(
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
                    .ok_or(NominatimError::Parsing(
                        "The reverse response returned an invalid name".into(),
                    ))?
                    .into(),
                geocoding["country"]
                    .as_str()
                    .ok_or(NominatimError::Parsing(
                        "The reverse response returned an invalid name".into(),
                    ))?
                    .into(),
            ));
        }

        Err(NominatimError::Parsing(
            "The reverse response did not contain the name or country".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    const BASE_URL: &'static str = "https://nominatim.openstreetmap.org/";

    #[test]
    fn client_construction() {
        let client = Client::builder().base_url(BASE_URL.into()).build();

        assert_eq!(BASE_URL, client.base_url.as_str());
    }

    #[tokio::test]
    async fn search_test() {
        let client = Client::builder().base_url(BASE_URL.into()).build();

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

        let expected_coordinates = Coordinates::try_new(56.1518, 10.210985).unwrap();
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }

    #[tokio::test]
    async fn search_list_test() {
        let client = Client::builder().base_url(BASE_URL.into()).build();

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
            Coordinates::try_new(56.1518, 10.210985).unwrap(),
            Coordinates::try_new(56.151714, 10.210944).unwrap(),
            Coordinates::try_new(56.149483, 10.20983).unwrap(),
        ];
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }

    #[tokio::test]
    async fn reverse_test() {
        let client = Client::builder().base_url(BASE_URL.into()).build();

        let coordinates = Coordinates::try_new(56.1518, 10.210985).unwrap();

        let reverse_result = client.reverse(coordinates).await;
        assert!(reverse_result.is_ok());

        let expected_response = ("Aarhus".to_string(), "Danmark".to_string());
        assert_eq!(reverse_result.unwrap(), expected_response);
    }
}
