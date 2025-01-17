use crate::{
    parameters::{ReverseParameters, ReverseResponse, SearchParameters, SearchResponse},
    Coordinate, Error,
};

pub struct Client {
    client: reqwest::Client,
    base_url: &'static str,
    email: &'static str,
}

impl Client {
    pub fn new(base_url: &'static str, email: &'static str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url,
            email: email,
        }
    }

    pub async fn search(&self, parameters: &SearchParameters) -> Result<Coordinate, Error> {
        let request = self
            .client
            .get(format!("{}/search", self.base_url))
            .query(parameters)
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

        Ok(Coordinate {
            lat: geometry_coordinates[1],
            lon: geometry_coordinates[0],
        })
    }

    pub async fn reverse(&self, parameters: &ReverseParameters) -> Result<(String, String), Error> {
        let request = self
            .client
            .get(format!("{}/reverse", self.base_url))
            .query(parameters)
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

    const BASE_URL: &'static str = "https://nominatim.openstreetmap.org";
    const EMAIL: &'static str = "user@company.com";

    #[test]
    fn client_construction() {
        let client = Client::new(BASE_URL, EMAIL);

        assert_eq!(BASE_URL, client.base_url);
        assert_eq!(EMAIL, client.email);
    }

    #[tokio::test]
    async fn search_test() {
        let client = Client::new(BASE_URL, EMAIL);

        let search = SearchParameters {
            street: "Spanien 1".to_string(),
            county: None,
            state: None,
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: None,
            email: EMAIL.to_string(),
            format: "geocodejson".to_string(),
        };

        let search_result = client.search(&search).await;
        println!("{:?}", search_result);
        assert!(search_result.is_ok());

        let expected_coordinates = Coordinate {
            lon: 10.210985,
            lat: 56.1518,
        };
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }

    #[tokio::test]
    async fn reverse_test() {
        let client = Client::new(BASE_URL, EMAIL);

        let coordinates = ReverseParameters {
            lon: 10.210985,
            lat: 56.1518,
            email: EMAIL.into(),
            format: "geocodejson".into(),
            zoom: 10,
        };

        let reverse_result = client.reverse(&coordinates).await;
        assert!(reverse_result.is_ok());

        let expected_response = ("Aarhus".to_string(), "Danmark".to_string());
        assert_eq!(reverse_result.unwrap(), expected_response);
    }
}
