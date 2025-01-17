use crate::{search::SearchParameters, Coordinate, Error};

use json;

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
        println!("{:?}", request);
        println!("{:?}", request.url());
        let response = self.client.execute(request).await?;

        let text = response.text().await?;
        println!("{}", text);
        let response_body = json::parse(text.as_str())?;

        // Ugly parsing of response here. We should deserialize
        let response_coordinates = &response_body["features"][0]["geometry"]["coordinates"];

        if response_coordinates.is_null() {
            return Err(Error::Parsing(json::JsonError::UnexpectedEndOfJson));
        }

        let mut coordinates = response_coordinates.members();
        Ok(Coordinate {
            latitude: coordinates
                .next()
                .ok_or(Error::Parsing(json::JsonError::wrong_type(
                    "Invalid coordinate length in API Response",
                )))?
                .as_f32()
                .ok_or(Error::Parsing(json::JsonError::wrong_type(
                    "Failed parsing coordinate from API",
                )))?,
            longitude: coordinates
                .next()
                .ok_or(Error::Parsing(json::JsonError::wrong_type(
                    "Invalid coordinate length in API Response",
                )))?
                .as_f32()
                .ok_or(Error::Parsing(json::JsonError::wrong_type(
                    "Failed parsing coordinate from API",
                )))?,
        })
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
            latitude: 10.210985,
            longitude: 56.1518,
        };
        assert_eq!(search_result.unwrap(), expected_coordinates);
    }
}
