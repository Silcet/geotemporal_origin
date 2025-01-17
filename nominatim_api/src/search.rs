use serde::Serialize;

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

impl Into<Vec<(String, String)>> for SearchParameters {
    fn into(self) -> Vec<(String, String)> {
        let mut params = vec![
            ("street".to_string(), self.street.replace(" ", "+")),
            ("city".to_string(), self.city.replace(" ", "+")),
            ("country".to_string(), self.country.replace(" ", "+")),
        ];

        if let Some(county) = self.county {
            params.push(("county".to_string(), county.replace(" ", "+")));
        }

        if let Some(state) = self.state {
            params.push(("state".to_string(), state.replace(" ", "+")));
        }

        if let Some(postalcode) = self.postalcode {
            params.push(("postalcode".to_string(), format!("{postalcode}")));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parameter_parsing() {
        let search = SearchParameters {
            street: "Spanien 1".to_string(),
            county: None,
            state: None,
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: None,
            email: "".to_string(),
            format: "geocodejson".to_string(),
        };

        let parameters: Vec<(String, String)> = search.into();

        let expected_parameters = vec![
            ("street".to_string(), "Spanien+1".to_string()),
            ("city".to_string(), "Aarhus".to_string()),
            ("country".to_string(), "Denmark".to_string()),
        ];

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn extra_parameter_parsing() {
        let search = SearchParameters {
            street: "Spanien 1".to_string(),
            county: Some("Aarhus Municipality".to_string()),
            state: None,
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: None,
            email: "".to_string(),
            format: "geocodejson".to_string(),
        };

        let parameters: Vec<(String, String)> = search.into();

        let mut expected_parameters = vec![
            ("street".to_string(), "Spanien+1".to_string()),
            ("city".to_string(), "Aarhus".to_string()),
            ("country".to_string(), "Denmark".to_string()),
            ("county".to_string(), "Aarhus+Municipality".to_string()),
        ];

        assert_eq!(expected_parameters, parameters);

        let search = SearchParameters {
            street: "Spanien 1".to_string(),
            county: Some("Aarhus Municipality".to_string()),
            state: Some("Central Denmark Region".to_string()),
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: None,
            email: "".to_string(),
            format: "geocodejson".to_string(),
        };

        expected_parameters.push(("state".to_string(), "Central+Denmark+Region".to_string()));

        let parameters: Vec<(String, String)> = search.into();

        assert_eq!(expected_parameters, parameters);

        let search = SearchParameters {
            street: "Spanien 1".to_string(),
            county: Some("Aarhus Municipality".to_string()),
            state: Some("Central Denmark Region".to_string()),
            city: "Aarhus".to_string(),
            country: "Denmark".to_string(),
            postalcode: Some(8000),
            email: "".to_string(),
            format: "geocodejson".to_string(),
        };

        expected_parameters.push(("postalcode".to_string(), "8000".to_string()));

        let parameters: Vec<(String, String)> = search.into();

        assert_eq!(expected_parameters, parameters);
    }
}
