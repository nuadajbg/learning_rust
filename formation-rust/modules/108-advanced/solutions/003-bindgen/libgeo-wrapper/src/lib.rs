#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod inner {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use inner::GeoCoordinate;

pub fn offset(from: GeoCoordinate, offset_lat: f64, offset_lng: f64) -> GeoCoordinate {
    unsafe { inner::offset(from, offset_lat, offset_lng) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let x = GeoCoordinate {
            latitude: 1.0,
            longitude: 2.0,
        };
        let y = offset(x, 2.0, 3.0);
        assert_eq!(y.latitude, 3.0);
        assert_eq!(y.longitude, 5.0);
    }
}
