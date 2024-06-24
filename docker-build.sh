set -ex
export MSYS_NO_PATHCONV=1

docker build . --tag rust-nightly-leptos:v1
docker run -it --init --name leptospiroza -v .:/app -w /app  rust-nightly-leptos:v1 sleep infinity