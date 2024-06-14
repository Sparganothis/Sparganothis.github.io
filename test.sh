#!/bin/bash
set -e

echo
echo "======================================"
echo "===  TEST DEFAULT PLATFORM WIN32  ===="
echo "======================================"
cargo test --package=webapp --no-default-features --features=ssr

echo
echo "======================================"
echo "=======    TEST WASM32 NODEJS    ======="
echo "======================================"
cargo test  --package=webapp --lib --target-dir=target/front --no-default-features --features=csr --target=wasm32-unknown-unknown


# echo
# echo "======================================"
# echo "=======    CARGO LEPTOS TEST    ======="
# echo "======================================"
# cargo leptos test
