use leptos::*;

use crate::client::game_board::GameBoard;
use crate::game::random::GameSeed;
use crate::game::tet::{GameReplaySegment, GameState};
use crate::server::api::websocket::SocketType;

#[component]
pub fn SpectatorGameBoard() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let state = create_rw_signal(GameState::new(&seed, 0));

    let leptos_use::UseWebsocketReturn {
        ready_state,
        message,
        send,
        open: _ws_open,
        close: _ws_close,
        ..
    } = leptos_use::use_websocket("/api/ws");
    let ws_status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == leptos_use::core::ConnectionReadyState::Open;

    let send2 = send.clone();
    let is_spectate_started = move || {
        if connected() {
            log::info!("we are ocnnected to wesbsock");
            let socket_type = SocketType::Specctate(uuid::Uuid::new_v4());
            let json = serde_json::to_string(&socket_type).expect("json never fail");
            send2(&json);
            return true;
        } else {
            false
        }
    };

    let get_segment = move || {
        if let Some(msg) = message.get() {
            if let Ok(segment) = serde_json::from_str(&msg) {
                Some(segment)
            } else {
                None
            }
        } else {
            None
        }
    };

    let memo_state = create_memo(move |_| {
        let segment = get_segment();
        match segment {
            Some(GameReplaySegment::Init(init)) => {
                state.update(|state_val| {
                    *state_val = GameState::new(&init.init_seed, init.start_time)
                });
                true
            }
            Some(GameReplaySegment::Update(slice)) => {
                state.update(|state_val| {
                    if let Err(e) = state_val.accept_replay_slice(&slice) {
                        log::warn!("error in accept_replay_slice() : {:?}", e);
                    }
                });
                true
            }
            Some(GameReplaySegment::GameOver) => {
                log::info!("got GameOver event; reply close and cloze websockat");
                let json = serde_json::to_string(&GameReplaySegment::GameOver).expect("json never fail");
                send(&json);
                _ws_close();
                true
                
            }
            None => {
                log::info!("websocket message is none!~");
                false
            }
        }
    });

    // let count = create_sse_signal::<GameState>("game_state");

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    log::info!("sse demo");
    view! {

        <GameBoard on_reset_game=on_reset game_state=state/>

        <p> Ws Status {ws_status} </p>
        <p> Is started: {move|| is_spectate_started()} </p>
        <p> Is ready: {move || memo_state.get()} </p>

        <p>"Receive message: " {move || format!("{:?}", message.get())}</p>


    }
}
