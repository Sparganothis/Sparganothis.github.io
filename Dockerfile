FROM rustlang/rust@sha256:8850381615662629f910564085d4e1b004fa7fe971edf9b30d9a058c5aece16f
RUN apt-get update -y && apt-get install -y vim wget curl tmux  nodejs npm git  mold clang lld --fix-missing
RUN npm install -g wasm-pack
RUN npm install -g sass
RUN  rustup default nightly && rustup update && rustup target add wasm32-unknown-unknown && rustup component add llvm-tools-preview && rustup component add rustc-codegen-cranelift-preview --toolchain nightly
RUN cargo install cargo-binstall
RUN cargo install trunk
RUN cargo install -f cargo-binutils 
RUN cargo binstall wasm-bindgen-cli --no-confirm
RUN cargo binstall wasm-pack --no-confirm
RUN cargo binstall cargo-leptos --no-confirm
RUN cargo binstall leptosfmt --no-confirm
RUN cargo binstall cargo-watch --no-confirm
