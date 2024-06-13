use leptos::*;

use crate::client::game_board::GameBoard;
use crate::game::tet::{GameReplaySegment, GameState};
use crate::server::api::websocket::SocketType;

use crate::server::api::websocket::*;

#[component]
pub fn WebsocketDemo() -> impl IntoView {
    let leptos_use::UseWebsocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = leptos_use::use_websocket("/api/ws");
    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == leptos_use::core::ConnectionReadyState::Open;

    let is_spectate_started = move || {
        if connected() {
            log::info!("we are ocnnected to wesbsock");
            let socket_type = SocketType::Specctate(uuid::Uuid::new_v4());
            let json = serde_json::to_string(&socket_type).expect("json never fail");
            send(&json);
            return true;
        } else {
            false
        }
    };

    let replay_state = move || {
        let msg = message();
        if let Some(msg) = msg {
            if let Ok(segment) = serde_json::from_str(&msg) {
                Some(segment)
            } else {
                None
            }
        } else {
            None
        }
    };

    // let s: ReadSignal<Option<Option<GameReplaySegment>>> =
    //     create_signal_from_stream(update_stream);
    //         |value| match value {
    //             Ok(value) => {
    //                 let json_string = value.1.data().as_string().unwrap();
    //                 let val: GameReplaySegment = serde_json::from_str(&json_string).unwrap();
    //                 if val.is_game_over() {
    //                     log::info!("game over but we didnt lcose")
    //                     // source.close();
    //                 }
    //                 Some(val)
    //             }
    //             Err(_e) => None,
    //         },
    //     ));

    //     on_cleanup(move || source.close());
    //     s
    // };

    let memo_state = create_memo(move |old_state: Option<&GameState>| {
        let segment = replay_state();
        match segment {
            Some(GameReplaySegment::Init(init)) => {
                log::info!("gamereplayinit{:?}", init);
                GameState::new(&init.init_seed, init.start_time)
            }
            Some(GameReplaySegment::Update(slice)) => {
                let mut state_val = old_state.unwrap().clone();
                if let Err(e) = state_val.accept_replay_slice(&slice) {
                    log::warn!("error in accept_replay_slice() : {:?}", e);
                }
                state_val
            }
            None => {
                let seed = [0; 32];
                GameState::new(&seed, 0)
            }
        }
    })
    .into_signal();

    // let count = create_sse_signal::<GameState>("game_state");

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    log::info!("sse demo");

    view! {

            <GameBoard on_reset_game=on_reset game_state=memo_state/>


            <p> {move|| format!("{}", is_spectate_started())} </p>

                <p>"Receive message: " {move || format!("{:?}", message.get())}</p>


    }
}
