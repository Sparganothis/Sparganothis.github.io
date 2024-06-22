use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    // routing::get,
    // Router,
};
use axum_extra::TypedHeader;
use game::api::websocket::{APIMethod, WebsocketAPIMessageRaw};
// use serde::Deserialize;

use std::borrow::Cow;
// use std::ops::ControlFlow;
use std::net::SocketAddr;
// use tower_http::services::ServeDir;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use axum_extra::headers;

use crate::database::tables::get_or_create_user_profile;

use super::session::Guest;
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
    guest: Guest,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("Websocket: `{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, guest))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, guest: Guest) {
    let user_info = (&guest).guest_data.clone();
    use futures::{sink::SinkExt, stream::StreamExt};
    let (mut sender, mut receiver) = socket.split();
    let (response_tx, mut response_rx) = tokio::sync::mpsc::channel(32);
    let (request_tx, mut request_rx) = tokio::sync::mpsc::channel(32);

    let mut send_task = tokio::spawn(async move {
        let mut cnt: usize = 0;
        while let Some(b) = response_rx.recv().await {
            cnt += 1;
            if let Err(e) = sender.send(Message::Binary(b)).await {
                log::warn!("could not send message becaue: {e}");
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(3)).await;
        }
        log::info!("Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            log::warn!("Could not send Close due to {e}, probably it is ok?");
        }
        cnt
    });

    let mut recv_task = tokio::spawn(async move {
        let mut cnt: usize = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            match msg {
                Message::Binary(b) => {
                    if let Err(e) = request_tx.send(b).await {
                        log::warn!("cannot put item in channel: {e}");
                        break;
                    }
                }
                Message::Close(_) => {
                    log::warn!("client closed.");
                    break;
                }
                _x => {
                    log::warn!("got bad msg type: {:?}", _x);
                    break;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(3)).await;
        }
        cnt
    });

    let mut process_task = tokio::spawn(async move {
        let mut cnt: usize = 0;
        while let Some(b) = request_rx.recv().await {
            cnt += 1;

            let b = match websocket_handle_request(b, (&user_info).clone()).await {
                Ok(b) => b,
                Err(e) => {
                    log::warn!("got error while handling websocket message: {:?}", e);
                    break;
                }
            };

            if let Err(e) = response_tx.send(b).await {
                log::warn!("cannot put response on channel: {e}");
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(3)).await;
        }
        cnt
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => log::warn!("{a} messages sent to {who} before stop"),
                Err(a) => log::warn!("Error sending messages {a:?}")
            }
            recv_task.abort();
            process_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => log::warn!("Received {b} messages from {who} before stop"),
                Err(b) => log::warn!("Error receiving messages {b:?}")
            }
            send_task.abort();
            process_task.abort();
        },

        rv_c = (&mut process_task) => {
            match rv_c {
                Ok(b) => log::warn!("Processed {b} messages before stop"),
                Err(b) => log::warn!("Error processing messages {b:?}")
            }
            send_task.abort();
            recv_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    log::info!("Websocket context {who} destroyed");
}
use game::api::user::GuestInfo;
pub async fn websocket_handle_request(
    b: Vec<u8>,
    user_id: GuestInfo,
) -> anyhow::Result<Vec<u8>> {
    use crate::backend::server_fn::*;
    use game::api::websocket::*;
    let user_id2 = user_id.clone();
    get_or_create_user_profile(&user_id2.user_id).unwrap();

    let msg: WebsocketAPIMessageRaw = bincode::deserialize(&b)
        .context("bincode deserialize fail for WebsocketAPIMessageRaw")?;
    let msg_type = msg._type.clone();
    log::info!(
        "handling request {:?} for userID {:?}",
        msg_type,
        user_id.user_id
    );
    let r: WebsocketAPIMessageRaw = match msg._type {
        WebsocketAPIMessageType::WhoAmI => {
            let callback = move |_, i| Ok(i);
            specific_sync_request::<WhoAmI>(msg, user_id, callback).await
        }
        WebsocketAPIMessageType::GetProfile => {
            specific_sync_request::<GetProfile>(msg, user_id, get_profile).await
        }
        WebsocketAPIMessageType::GitVersion => {
            specific_sync_request::<GitVersion>(msg, user_id, git_version).await
        }

        WebsocketAPIMessageType::CreateNewGameId => {
            specific_sync_request::<CreateNewGameId>(msg, user_id, create_new_game_id)
                .await
        }
        WebsocketAPIMessageType::AppendGameSegment => {
            specific_sync_request::<AppendGameSegment>(
                msg,
                user_id,
                append_game_segment,
            )
            .await
        }

        WebsocketAPIMessageType::GetSegmentCount => {
            specific_sync_request::<GetSegmentCount>(msg, user_id, get_segment_count)
                .await
        }
        WebsocketAPIMessageType::GetAllSegments => {
            specific_sync_request::<GetAllSegments>(
                msg,
                user_id,
                get_all_segments_for_game,
            )
            .await
        }
        WebsocketAPIMessageType::GetLastFullGameState => {
            specific_sync_request::<GetLastFullGameState>(
                msg,
                user_id,
                get_last_full_game_state,
            )
            .await
        }
        WebsocketAPIMessageType::GetAllGames => {
            specific_sync_request::<GetAllGames>(msg, user_id, get_all_games).await
        }
        WebsocketAPIMessageType::GetAllCustomGames => {
            specific_sync_request::<GetAllCustomGames>(msg, user_id, get_all_gustom)
                .await
        }
        WebsocketAPIMessageType::GetCustomGame => {
            specific_sync_request::<GetCustomGame>(msg, user_id, get_gustom_game).await
        }
        WebsocketAPIMessageType::UpdateCustomGame => {
            specific_sync_request::<UpdateCustomGame>(msg, user_id, update_custom_game)
                .await
        }
        WebsocketAPIMessageType::GetRandomWord => {
            specific_sync_request::<GetRandomWord>(msg, user_id, random_word2).await
        }
    }
    .context(format!("specific handler {:?}", msg_type))?;

    log::info!(
        "sending response {:?} for userID {:?}",
        msg_type,
        user_id2.user_id
    );
    Ok(bincode::serialize(&r).context("bincode never fail")?)
}
use anyhow::Context;
pub async fn specific_sync_request<T: APIMethod>(
    request_msg: WebsocketAPIMessageRaw,
    guest_info: GuestInfo,
    callback: impl Fn(T::Req, GuestInfo) -> anyhow::Result<T::Resp>
        + std::marker::Sync
        + std::marker::Send
        + 'static,
) -> anyhow::Result<WebsocketAPIMessageRaw> {
    if !request_msg._type.eq(&T::TYPE) {
        anyhow::bail!("wrong type dispatched");
    }
    if !request_msg.is_req {
        anyhow::bail!("message is not request");
    }
    let request: T::Req =
        bincode::deserialize(&request_msg.data).context("bincode never fail")?;

    let response: anyhow::Result<T::Resp> =
        tokio::task::spawn_blocking(move || callback(request, guest_info))
            .await
            .context("tokio never fail")?;
    let response = response.map_err(|e| format!("websocket method error: {e}"));

    Ok(WebsocketAPIMessageRaw {
        id: request_msg.id,
        _type: request_msg._type,
        is_req: false,
        data: bincode::serialize(&response).context("bincode never fail")?,
    })
}

// async fn process_socket(socket: &mut WebSocket, socket_type: SocketType) {
//     match socket_type {
//         SocketType::Specctate(game_id) => {
//             log::warn!("spectate started game id = {game_id} ");
//             process_replay_spectate(game_id, socket).await;
//         }
//         SocketType::Game1V1 => {
//             log::warn!("socket type 1v1 not impl;");
//         }
//     }

//     if let Err(e) = socket
//         .send(Message::Close(Some(CloseFrame {
//             code: axum::extract::ws::close_code::NORMAL,
//             reason: Cow::from("Goodbye"),
//         })))
//         .await
//     {
//         log::warn!("Could not send Close due to {e}, probably it is ok?");
//     } else {
//         log::info!("Goodbyte.");
//     }
// }

// async fn process_replay_spectate(_game_id: uuid::Uuid, socket: &mut WebSocket) {
//     // use axum::response::sse::{Event, Sse};
//     // use futures::stream::{self, Stream};
//     // use std::{convert::Infallible, time::Duration};

//     use game::tet::*;
//     use game::timestamp::get_timestamp_now_nano;
//     // use std::collections::VecDeque;

//     let mut maybe_state: Option<GameState> = None;

//     let mut is_over = false;

//     loop {
//         let mut new_segments = Vec::<GameReplaySegment>::new();

//         if let Some(mut state) = (&maybe_state).clone() {
//             let action = TetAction::random();
//             let t2 = get_timestamp_now_nano();
//             let _ = state.apply_action_if_works(action, t2);
//             if state.game_over {
//                 is_over = true;
//             }
//             maybe_state = Some(state.clone());

//             let new_slice = maybe_state
//                 .as_ref()
//                 .unwrap()
//                 .replay
//                 .replay_slices
//                 .last()
//                 .unwrap()
//                 .clone();
//             new_segments.push(GameReplaySegment::Update(new_slice));
//         } else {
//             let seed: [u8; 32] = [0; 32];
//             maybe_state = Some(GameState::new(&seed, get_timestamp_now_nano()));
//             new_segments.push(GameReplaySegment::Init(
//                 maybe_state.as_ref().unwrap().replay.clone(),
//             ));
//         }

//         for segment in new_segments {
//             let json = serde_json::to_string(&segment).expect("json never fail");
//             if let Err(_e) = socket.send(Message::Text(json)).await {
//                 log::warn!(
//                     "game {:?}: ERROR SOCKET SEND GAMME SSLICE  BAD HAPPEN",
//                     _game_id
//                 );
//                 return;
//             }
//         }

//         if is_over {
//             log::info!("game {:?}: got segment with game over, cloze", _game_id);
//             let segment = GameReplaySegment::GameOver;
//             let json = serde_json::to_string(&segment).expect("json never fail");
//             let _ = socket.send(Message::Text(json)).await;
//             let _ = socket.recv().await;
//             return;
//         }
//         tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//     }
// }
