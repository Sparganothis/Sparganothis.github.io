

# todo

## friendly match
- button to generate friendly match
- generate link unique with secret key
- first user sends link to second user
- second user when visiting link is auto-matchmake with first user
- match starts

## private / public rooms
- create public/private roomm
- list all publicc/private room
- share link to public/private roomm; other users when clicking link are joined to room
- auto-exit room at disconnect
- auto-ccreate mmatrix room for game room
- connect matrix chat with game using SSO/OIDC/rust-???
- create public matrix app for gamme

## fix public matchmmake
- timeout 30s - if not found public match, exit and try again
- button to abort public matchmake - run automatically on_cleanup in leptos + BIG RED BUTTON "give up"
- display recent mmamtchmake entry - usernaem + seconds ago + successful matches + past match result (like CS kill feed)
- block cconcurrent matchmake from 2 tabs -- fix error

## user managemment, usage tracking
- feedback mamtrix bot + global mmatrix bot
  - start from "matrix commmand bot"
  - https://github.com/matrix-org/matrix-rust-sdk/blob/main/examples/command_bot/src/main.rs
  - report feedback in UI
  - all feedback msg go to me
  - when I reply to the matrix bot, user gets response in their chet
- connect with mmatrix for general chat
  - iframe with #public channel
  - 

- user account creation
- login with other device
   - first device gives you 6 digit code / QR
   - second device scan / enter code/QR
   - click OK on both device
- global dashboard track
  - games
  - pageviews/logins
  - matches
  - id country counter
  - show flag in profile

## mmultiplayer + singleplayer
- win game (40 lines)
  lobby buttons:
    - LABL TEXT DECRIPTION: "CLEAR 40 LINES SHORTEST TIME SCORE DONT MMATTER GOTTA GO FAST"
    - PERSONAL BEST DISPLAY
    - LINK LEADERBOARDS
    - START BUTTON
    - OPTIONS:
          - LEVEL /SPEED
          - 40 lines/69 / 420
          - CUSTOMIZER UI

- win game (blitz)
  lobby buttons:
    - LABL TEXT DECRIPTION: "GET BIG SCORE IN 2 MIN! LEVEL UP! DO NONT DIE!"
    - PERSONAL BEST DISPLAY
    - LINK LEADERBOARDS
    - START BUTTON
    - OPTIONS:
          - LEVEL /SPEED
          - SLECT 1min/2min/4min
          - CUSTOMIZER UI

- train gamemmode: undo/redo pcs


- lose game (disconnect 15s)
- lose game (abandon button)


## build

- sound effect when build cvompletted

## 



## extra feature : replayt
- show gamme states about #6666 
  - keep skip list (1/ 100 in memory) and when going backwards, recompute prev list

## BOT/PERF
- OPTIMIZE GameState REpresentation to uyse bvit vector / int mask / all mamtrix in i128

- READ &* PARSE TETio SAVE FILE FORMAT JSON
- SAMPLE BOT FROM PYTHON SCRIPT 

- OPTIMIZE: SAMPLE FROM TORCHSCRIPT
- -addd more info to game state:
  - wordpress bot things: # holes, bumpiness, max height, if line clear


## PAGINAITE ALL TABLES 

- API ROUTES IGNORE _PAG PARAM

## custom game / mspaint
- -fix mmspaint nextpcps selector & use selected pcs when playing mspainyt (not deterministic random next pcs)
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









































## DELETE LEPTONIC CSS

- delete css from leptonic




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


### matrix app/integrration to challenge chatmember to game
- slack marketplace app to add and challenge to game
- mattermost marketplace
- ms teams & ms store
- "DUEL" / "CHALLENGE"