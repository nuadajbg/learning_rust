#!/bin/bash

SCRIPT="$(readlink -f "$0")"
DIR="$(dirname "$SCRIPT")"

LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$DIR/../target/debug" "$DIR/main"
