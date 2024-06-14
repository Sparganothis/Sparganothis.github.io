#!/bin/bash
set -e

bash ./build.sh

echo
echo "======================================"
echo "===  CARGO LEPTOSPIROZA WATCH    ===="
echo "======================================"
cargo-leptos watch
