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
