![](./.docs/Screenshot.png)


# GYM / GYMNASIUMM ENVIRONMMENT (python)

```
pip install sparganothis_vim

```

[EXAMPLE: test.py](./sparganothis_vim/test.py)


# HOW TO COMPILE

- install "rustup" and "nodejs"
- install "vscode" including "rust-analyzer" plugin
- install "git for windows" including "bash"
- install "sass" using npm: `npm install -g sass`
- run commands in vscode bash terminal



```
rustup default nightly
rustup update nightly
rustup target add wasm32-unknown-unknown

cargo install cargo-binstall
cargo binstall trunk
cargo binstall wasm-bindgen-cli --no-confirm
cargo binstall wasm-pack --no-confirm
cargo binstall cargo-leptos --no-confirm
cargo binstall leptosfmt --no-confirm
cargo binstall cargo-watch --no-confirm
cargo binstall cargo-edit --no-confirm
cargo install -f cargo-binutils
rustup component add llvm-tools-preview


bash test.sh
bash run-client.sh
bash run-server.sh
```
