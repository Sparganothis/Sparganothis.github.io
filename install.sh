#!/bin/bash

set -ex
# TODO RUN WITH SUDO
sudo apt-get update -y && sudo apt-get install -y vim wget curl tmux  nodejs npm git  mold clang lld --fix-missing
sudo npm install -g wasm-pack
sudo npm install -g sass
( cd client; rustup update && rustup target add wasm32-unknown-unknown )

# TOD INSTAL L RUSTUP AND RUST BEFORE RUNNING THIS
# TODO RUN THIS ONLY ON CCLIENT WITH USER non-root
 rustup default nightly && rustup update && rustup target add wasm32-unknown-unknown && rustup component add llvm-tools-preview && rustup component add rustc-codegen-cranelift-preview --toolchain nightly
cargo install cargo-binstall
cargo binstall trunk
cargo install -f cargo-binutils 
cargo binstall wasm-bindgen-cli --no-confirm
cargo binstall wasm-pack --no-confirm
cargo binstall cargo-leptos --no-confirm
cargo binstall leptosfmt --no-confirm
cargo binstall cargo-watch --no-confirm
