use nominatim_api::{client::Client, Coordinates, Location, Result};

use std::iter::zip;

struct GeotemporalAverager {
    api_client: Client,
}

impl GeotemporalAverager {
    pub fn new(base_url: String) -> Self {
        Self {
            api_client: Client::builder().base_url(base_url.into()).build(),
        }
    }

    async fn average_locations(&self, locations: Vec<(f32, Location)>) -> Result<Coordinates> {
        let (times, locations): (Vec<f32>, Vec<Location>) = locations.into_iter().unzip();
        let coordinates = self.api_client.search_list(locations).await?;

        let (total_time, weighted_coordinates) = zip(times.into_iter(), coordinates.into_iter())
            .fold(
                (0.0f32, Coordinates::try_new(0.0, 0.0).unwrap()),
                |mut acc, (time, coordinates)| {
                    acc.0 += time;
                    acc.1 = acc.1 + coordinates * time;
                    acc
                },
            );

        Ok(weighted_coordinates / total_time)
    }

    pub async fn get_geotemporal_origin(
        &self,
        locations: Vec<(f32, Location)>,
    ) -> Result<(String, String)> {
        let average_coordinates = self.average_locations(locations).await?;

        self.api_client.reverse(average_coordinates).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    const BASE_URL: &'static str = "https://nominatim.openstreetmap.org";

    #[tokio::test]
    async fn average_locations_line_test() {
        let locations = vec![
            (
                5.0f32,
                Location {
                    street: "Spanien 1".into(),
                    city: "Aarhus".into(),
                    country: "Denmark".into(),
                    ..Default::default()
                },
            ),
            (
                5.0f32,
                Location {
                    street: "Spanien 11".into(),
                    city: "Aarhus".into(),
                    country: "Denmark".into(),
                    ..Default::default()
                },
            ),
        ];
        let expected_coordinates = Coordinates::try_new(56.15161, 10.2109025).unwrap();

        let average = GeotemporalAverager::new(BASE_URL.into())
            .average_locations(locations)
            .await;
        assert!(average.is_ok());

        assert_eq!(average.unwrap(), expected_coordinates)
    }

    #[tokio::test]
    async fn average_locations_weighted_test() {
        let locations = vec![
            (
                0.0f32,
                Location {
                    street: "Spanien 1".into(),
                    city: "Aarhus".into(),
                    country: "Denmark".into(),
                    ..Default::default()
                },
            ),
            (
                5.0f32,
                Location {
                    street: "Spanien 11".into(),
                    city: "Aarhus".into(),
                    country: "Denmark".into(),
                    ..Default::default()
                },
            ),
        ];
        let expected_coordinates = Coordinates::try_new(56.15142, 10.21082).unwrap();

        let average = GeotemporalAverager::new(BASE_URL.into())
            .average_locations(locations)
            .await;
        assert!(average.is_ok());

        assert_eq!(average.unwrap(), expected_coordinates)
    }
}
