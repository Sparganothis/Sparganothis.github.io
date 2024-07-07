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


# todo

## custom game / mspaint
- save a difgferent strucct "GameBoardCustomM" that only contains editable things in mspaint.exe (avoid dropping old custom game baords)

## bugs
- cannot replay bot games (stuck on first move)
- if spectating eded game, UI blocked on gamme over instead of redirecting to replay
- if spectating match, UI should refresh ended games with the replay
- 
- 1v1 online complete disaster.
- 1v1 + man vs car : if open seccond tab on /mathc/ link, it's allowed to play, should be redirect  to /spectate-match/
- man vs. car: replay is not working for Bot games.



## SINGLEPLAYER

- SINGLEPLAYE$R: TIMER  + 40 LINE WORLD RECCORD + BLIETZ + "PUZZLE MODE" + TUTORIAL MODE

- change all Get* api message to return None for "not found" -- is not error (custom game / mmspaint on randomm name);
- https://github.com/goldfire/howler.js/tree/master/examples/sprite
- rotate tests fromm mwiki
  - finesse https://harddrop.com/wiki/0G_60_Hz_SRS_Movement_Finesse
  - SRS https://tetris.fandom.com/wiki/SRS

### feature = singleplayer

- leaderboard + world record



### 1V1 ONLINE

- Combo GARBAGE - bara + send lines between bots


## grafic














































## audio

- make tutros bot sound like https://soundcloud.com/funk-it-blog-1/kris-bowers-forever-spring-41814-boston-ma-scullers-jazz-club
- mmake tatrus bot sound like https://soundcloud.com/bigupmagazine/big-up-podcast-89-commodo
- and like https://soundcloud.com/lucas-goret/armenian
- winamp
- remmember in private storeage all winamp history and user ignore / skpi soundtrack
- when song changes, put notiifcation in corner like nfss
- different cocmbo audio TSpin JSpin Back-To_Back-X Tetris
- menu option to disable soft/auto-soft-drop sound
- eacch different menu option sound has itws own slider
- option for decreasing volume for repeating sound

### whatever

  winamp music stream play pause music in corner of screen(youtube playlist)