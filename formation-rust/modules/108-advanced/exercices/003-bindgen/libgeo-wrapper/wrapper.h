/**
 * Geographic coordinates for a point
 */
struct GeoCoordinate {
    /**
     * Latitude coordinate
     */
    double latitude;
    /**
     * Longitude coordinate
     */
    double longitude;
};

/**
 * Computes a new geographic coordinate from the specified one,
 * and latitude, longitude offsets
 */
struct GeoCoordinate offset(struct GeoCoordinate from, double offset_lat, double offset_lng);
