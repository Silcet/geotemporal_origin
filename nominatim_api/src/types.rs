use crate::{NominatimError, Result};

use core::ops::{Add, Div, Mul, RangeInclusive};
use serde::Serialize;

const LAT_RANGE: RangeInclusive<f32> = -90.0..=90.0;
const LON_RANGE: RangeInclusive<f32> = -180.0..=180.0;

#[derive(Debug, PartialEq, Serialize)]
pub struct Coordinates {
    lat: f32,
    lon: f32,
}

impl Coordinates {
    pub fn try_new(lat: f32, lon: f32) -> Result<Self> {
        if !LAT_RANGE.contains(&lat) {
            return Err(NominatimError::InvalidCoordinate(lat, "latitude".into()));
        }
        if !LON_RANGE.contains(&lon) {
            return Err(NominatimError::InvalidCoordinate(lon, "longitude".into()));
        }

        Ok(Self { lat: lat, lon: lon })
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            lat: self.lat + other.lat,
            lon: self.lon + other.lon,
        }
    }
}

impl Mul<f32> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            lat: self.lat * rhs,
            lon: self.lon * rhs,
        }
    }
}

impl<'a> Mul<&'a f32> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: &'a f32) -> Self::Output {
        Coordinates::mul(self, *rhs)
    }
}

impl Div<f32> for Coordinates {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            lat: self.lat / rhs,
            lon: self.lon / rhs,
        }
    }
}

impl<'a> Div<&'a f32> for Coordinates {
    type Output = Self;

    fn div(self, rhs: &'a f32) -> Self::Output {
        Coordinates::div(self, *rhs)
    }
}

#[derive(Serialize, Default, Clone)]
pub struct Location {
    pub street: String,
    pub city: String,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postalcode: Option<u16>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    const VALID_LATS: &[f32; 5] = &[90.0, 45.0, 0.0, -45.0, -90.0];
    const INVALID_LATS: &[f32; 5] = &[91.0, 145.0, 180.0, -100.0, -90.1];
    const VALID_LONS: &[f32; 5] = &[180.0, 135.0, 0.0, -90.0, -180.0];
    const INVALID_LONS: &[f32; 5] = &[181.0, 270.0, 360.0, -200.0, -180.1];

    #[test]
    fn try_new_test() {
        VALID_LATS
            .iter()
            .zip(VALID_LONS.iter())
            .for_each(|(lat, lon)| assert!(Coordinates::try_new(*lat, *lon).is_ok()));
        VALID_LATS
            .iter()
            .zip(INVALID_LONS.iter())
            .for_each(|(lat, lon)| assert!(Coordinates::try_new(*lat, *lon).is_err()));
        INVALID_LATS
            .iter()
            .zip(VALID_LONS.iter())
            .for_each(|(lat, lon)| assert!(Coordinates::try_new(*lat, *lon).is_err()));
        INVALID_LATS
            .iter()
            .zip(INVALID_LONS.iter())
            .for_each(|(lat, lon)| assert!(Coordinates::try_new(*lat, *lon).is_err()));
    }

    #[test]
    fn add_test() {
        let coordinates_tuple: Vec<(&f32, &f32)> =
            zip(VALID_LATS.iter(), VALID_LONS.iter()).collect();
        let mut shifted_coordinates_tuple = coordinates_tuple.clone();
        let first = shifted_coordinates_tuple.swap_remove(0);
        shifted_coordinates_tuple.push(first);

        zip(
            coordinates_tuple.into_iter(),
            shifted_coordinates_tuple.into_iter(),
        )
        .for_each(|((lat1, lon1), (lat2, lon2))| {
            let c1 = Coordinates::try_new(*lat1, *lon1).unwrap();
            let c2 = Coordinates::try_new(*lat2, *lon2).unwrap();

            let c = c1 + c2;

            assert_eq!(c.lat, lat1 + lat2);
            assert_eq!(c.lon, lon1 + lon2);
        })
    }

    #[test]
    fn mul_test() {
        zip(VALID_LATS.iter(), VALID_LONS.into_iter())
            .zip(3..=8)
            .for_each(|((lat, lon), mul)| {
                let c = Coordinates::try_new(*lat, *lon).unwrap() * mul as f32;

                assert_eq!(c.lat, lat * mul as f32);
                assert_eq!(c.lon, lon * mul as f32);
            })
    }

    #[test]
    fn div_test() {
        zip(VALID_LATS.iter(), VALID_LONS.into_iter())
            .zip(3..=8)
            .for_each(|((lat, lon), div)| {
                let c = Coordinates::try_new(*lat, *lon).unwrap() / div as f32;

                assert_eq!(c.lat, lat / div as f32);
                assert_eq!(c.lon, lon / div as f32);
            })
    }
}
