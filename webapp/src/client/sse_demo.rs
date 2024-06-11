use leptos::*;

use crate::client::game_board::GameBoard;
use crate::game::tet::{GameReplaySegment, GameState};


#[component]
pub fn SseDeom() -> impl IntoView {

    let replay_state = {
        use futures::StreamExt;

        let mut source = gloo_net::eventsource::futures::EventSource::new("/api/events")
            .expect("couldn't connect to SSE stream");
        let s: ReadSignal<Option<Option<GameReplaySegment>>> =
            create_signal_from_stream(source.subscribe("message").unwrap().map(
                |value| match value {
                    Ok(value) => {
                        let json_string = value.1.data().as_string().unwrap();
                        let val: GameReplaySegment = serde_json::from_str(&json_string).unwrap();
                        if val.is_game_over() {
                            log::info!("game over but we didnt lcose")
                            // source.close();
                        }
                        Some(val)
                    }
                    Err(_e) => None,
                },
            ));

        on_cleanup(move || source.close());
        s
    };

    let memo_state = create_memo(move |old_state: Option<&GameState>| {
        let  segment = replay_state.get().unwrap_or(None);
        match segment {
            Some(GameReplaySegment::Init(init)) => {
                GameState::new(&init.init_seed, init.start_time)
            },            
            Some(GameReplaySegment::Update(slice)) => {
                let mut state_val  = old_state.unwrap().clone();
                if let Err(e) = state_val.accept_replay_slice(&slice) {
                    log::warn!("error in accept_replay_slice() : {:?}", e);
                }
                state_val
            },
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
    view! { <GameBoard on_reset_game=on_reset game_state=memo_state/> }
}
