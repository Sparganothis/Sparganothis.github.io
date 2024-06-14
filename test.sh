#!/bin/bash
set -e

cd webapp

echo
echo "======================================"
echo "===  TEST DEFAULT PLATFORM WIN32  ===="
echo "======================================"
cargo test -- --nocapture --test-threads 1

echo
echo "======================================"
echo "=======    TEST WASM32 NODE    ======="
echo "======================================"
cargo test --target wasm32-unknown-unknown
