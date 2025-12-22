#!/bin/sh
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root or with sudo!" >&2
    exit 1
fi

chmod +x ./target/aarch64-unknown-linux-gnu/release/indra
./target/aarch64-unknown-linux-gnu/release/indra

git restore *