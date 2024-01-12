#include "libgeo.h"
#include <stdio.h>

int main() {
    struct GeoCoordinate x;
    x.latitude = 1.0;
    x.longitude = 2.0;
    struct GeoCoordinate y = offset(x, 2.0, 3.0);
    printf("{latitude: %f, longitude: %f}\n", y.latitude, y.longitude);
    return 0;
}
