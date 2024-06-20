#!/bin/bash
set -ex
cd server
cargo watch -x run -w ../game/ -i ../game/target -w . -i server_data -i target