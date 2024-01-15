/// Geographic coordinates for a point
#[repr(C)]
pub struct GeoCoordinate {
    /// Latitude coordinate
    latitude: f64,
    /// Longitude coordinate
    longitude: f64,
}

/// Computes a new geographic coordinate from the specified one, and latitude, longitude offsets
#[no_mangle]
#[export_name = "offset"]
pub fn offset(from: GeoCoordinate, offset_lat: f64, offset_lng: f64) -> GeoCoordinate {
    GeoCoordinate {
        latitude: from.latitude + offset_lat,
        longitude: from.longitude + offset_lng,
    }
}

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
