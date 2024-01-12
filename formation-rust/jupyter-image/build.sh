#!/bin/bash

# Ensures that the build environment exists, as well as the necessary base images

SCRIPT="$(readlink -f "$0")"
DIR="$(dirname "$SCRIPT")"

docker build --tag "cenotelie/jupyter-kernel-rust:latest" --rm "$DIR"
