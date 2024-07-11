#!/bin/bash
set -ex
cd server
cargo watch --why --delay 3 -s "bash -exc ' time cargo build; cargo run ' " -w ../game/ -i ../game/target -w . -i server_data -i target  -w ../VERSION