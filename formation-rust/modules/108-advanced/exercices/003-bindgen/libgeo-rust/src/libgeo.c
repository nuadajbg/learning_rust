#include "libgeo.h"

struct GeoCoordinate offset(struct GeoCoordinate from, double offset_lat, double offset_lng) {
    struct GeoCoordinate result;
    result.latitude = from.latitude + offset_lat;
    result.longitude = from.longitude + offset_lng;
    return result;
}
