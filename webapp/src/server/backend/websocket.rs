//! Example websocket server.
//!
//! Run the server with
//! ```not_rust
//! cargo run -p example-websockets --bin example-websockets
//! ```
//!
//! Run a browser client with
//! ```not_rust
//! firefox http://localhost:3000
//! ```
//!
//! Alternatively you can run the rust client (showing two
//! concurrent websocket connections being established) with
//! ```not_rust
//! cargo run -p example-websockets --bin example-client
//! ```
// use serde::Serialize;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    // routing::get,
    // Router,
};
use axum_extra::TypedHeader;
// use serde::Deserialize;

use std::borrow::Cow;
// use std::ops::ControlFlow;
use std::net::SocketAddr;
// use tower_http::services::ServeDir;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use axum_extra::headers;
//allows to split the websocket stream into separate TX and RX branches
// use futures::{sink::SinkExt, stream::StreamExt};

// use crate::server::api::user::{GuestInfo, UserProfile};

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("Websocket: `{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    // use crate::server::backend::session::extract_g/uest_data;

    // log::info!("init ws");
    // let gues/t_id = extract_guest_data().await.expect("no guest info after exctract");
    // let  user_profile  = crate::server::api::user::get_profile(guest_id.user_id).await.expect("usetr  profile does not exist");

    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

use super::super::api::websocket::*;
/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    // if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
    //     log::info!("Pinged {who}...");
    // } else {
    //     log::info!("Could not send ping {who}!");
    //     return;
    // }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            log::info!("got message {msg:?}");
            match msg {
                Message::Text(d) => {
                    if let Ok(socket_type) = serde_json::from_str::<SocketType>(&d) {
                        process_socket(&mut socket, socket_type).await;
                        return;
                    }
                }
                Message::Close(_c) => {
                    return;
                }
                _ => {
                    log::warn!("message  not  match --> exit ws");
                    return;
                }
            }
        } else {
            log::info!("client {who} abruptly disconnected");
            return;
        }
    }

    // returning from the handler closes the websocket connection
    log::info!("Websocket context {who} destroyed");
}

async fn process_socket(socket: &mut WebSocket, socket_type: SocketType) {
    match socket_type {
        SocketType::Specctate(game_id) => {
            log::warn!("spectate started game id = {game_id} ");
            process_replay_spectate(game_id, socket).await;
        }
        SocketType::Game1V1 => {
            log::warn!("socket type 1v1 not impl;");
        }
    }

    if let Err(e) = socket
        .send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::NORMAL,
            reason: Cow::from("Goodbye"),
        })))
        .await
    {
        log::warn!("Could not send Close due to {e}, probably it is ok?");
    } else {
        log::info!("Goodbyte.");
    }
}

async fn process_replay_spectate(_game_id: uuid::Uuid, socket: &mut WebSocket) {
    // use axum::response::sse::{Event, Sse};
    // use futures::stream::{self, Stream};
    // use std::{convert::Infallible, time::Duration};

    use crate::game::tet::*;
    use crate::game::timestamp::get_timestamp_now_nano;
    // use std::collections::VecDeque;

    let mut maybe_state: Option<GameState> = None;

    let mut is_over = false;

    loop {
        let mut new_segments = Vec::<GameReplaySegment>::new();

        if let Some(mut state) = (&maybe_state).clone() {
            let action = TetAction::random();
            let t2 = get_timestamp_now_nano();
            let _ = state.apply_action_if_works(action, t2);
            if state.game_over {
                is_over = true;
            }
            maybe_state = Some(state.clone());

            let new_slice = maybe_state
                .as_ref()
                .unwrap()
                .replay
                .replay_slices
                .last()
                .unwrap()
                .clone();
            new_segments.push(GameReplaySegment::Update(new_slice));
        } else {
            let seed: [u8; 32] = [0; 32];
            maybe_state = Some(GameState::new(&seed, get_timestamp_now_nano()));
            new_segments.push(GameReplaySegment::Init(
                maybe_state.as_ref().unwrap().replay.clone(),
            ));
        }

        for segment in new_segments {
            let json = serde_json::to_string(&segment).expect("json never fail");
            if let Err(_e) = socket.send(Message::Text(json)).await {
                log::warn!(
                    "game {:?}: ERROR SOCKET SEND GAMME SSLICE  BAD HAPPEN",
                    _game_id
                );
                return;
            }
        }

        if is_over {
            log::info!("game {:?}: got segment with game over, cloze", _game_id);
            let segment = GameReplaySegment::GameOver;
            let json = serde_json::to_string(&segment).expect("json never fail");
            let _ = socket.send(Message::Text(json)).await;
            let _ = socket.recv().await;
            return;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
