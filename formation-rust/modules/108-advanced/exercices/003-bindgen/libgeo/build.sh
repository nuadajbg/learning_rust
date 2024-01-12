#!/bin/bash

SCRIPT="$(readlink -f "$0")"
DIR="$(dirname "$SCRIPT")"

(cd "$DIR"; gcc -Wall -fPIC -c libgeo.c)
(cd "$DIR"; gcc -shared libgeo.o -o libgeo.so)
(cd "$DIR"; ar crs libgeo.a libgeo.o)
