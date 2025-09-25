#!/usr/bin/env bash

cargo build --release
cp -f $(dirname "$0")/target/release/cs2 /usr/local/bin/cs2
