#!/bin/bash

SCRIPT="$(readlink -f "$0")"
DIR="$(dirname "$SCRIPT")"

(cd "$DIR"; gcc -Wall main.c -o main -lgeo -L "$DIR/../target/debug/")
chmod +x "$DIR/main"
