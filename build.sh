#!/bin/bash
set -e

# echo
# echo "======================================"
# echo "===  BUILD    SERVER   HOST  ===="
# echo "======================================"
# (
# set -x
# cargo build --package=webapp --bin=web-server --no-default-features --features=ssr 
# )


# echo
# echo "======================================"
# echo "===  BUILD  CLIENT   WASM     ===="
# echo "======================================"
# (
# set -x
# cargo build --package=webapp --lib --target=wasm32-unknown-unknown --no-default-features --features=csr --target-dir=target/front
# )



echo
echo "======================================"
echo "===  BUILD  SERVER + CLIENT  ===="
echo "======================================"
(
set -x
cargo leptos build
)
