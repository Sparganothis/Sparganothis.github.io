![](./docs/Screenshot.png)


# HOW TO COMPILE

- install "rustup" and "nodejs"
- install "vscode" including "rust-analyzer" plugin
- install "git for windows" including "bash"
- run commands in vscode bash terminal



```
rustup default stable
rustup update stable
rustup target add wasm32-unknown-unknown

cargo install cargo-binstall
cargo binstall wasm-bindgen-cli --no-confirm
cargo binstall wasm-pack --no-confirm
cargo binstall cargo-leptos --no-confirm
cargo binstall leptosfmt --no-confirm

bash test.sh
bash run-server.sh
```


# todo
- rotate SRS
- rotate tests fromm mwiki
  - finesse https://harddrop.com/wiki/0G_60_Hz_SRS_Movement_Finesse
  - SRS https://tetris.fandom.com/wiki/SRS

### feature = singleplayer

- 40 line singleplayer + timer
- leaderboard + server replay + world record
- combo + TSPIN + Score

### 1V1 CPPU

- COMBO GARBAGE

### 1V1 ONLINE

- GARBAGE


## grafic

- singleplayer - make window centered
- hold / next: each pcs has its own centered board