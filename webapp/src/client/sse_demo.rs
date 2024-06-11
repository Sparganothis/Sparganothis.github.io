use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub mod ssr_imports {
    pub use broadcaster::BroadcastChannel;
    pub use once_cell::sync::OnceCell;
    pub use std::sync::atomic::{AtomicI32, Ordering};

    pub static COUNT: AtomicI32 = AtomicI32::new(0);

    lazy_static::lazy_static! {
        pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    }

    static LOG_INIT: OnceCell<()> = OnceCell::new();

    pub fn init_logging() {
        LOG_INIT.get_or_init(|| {
            simple_logger::SimpleLogger::new().env().init().unwrap();
        });
    }
}

#[server]
pub async fn adjust_server_count(delta: i32, msg: String) -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}
use leptos::*;

use crate::client::game_board::GameBoard;
use crate::game::tet::{GameReplay, GameState};

#[component]
pub fn SseDeom() -> impl IntoView {
    // leptos_sse::provide_sse("/api/events").unwrap();
    // Create sse signal
    // let replay_state: ReadSignal<GameReplay> = create_sse_signal::<GameReplay>("game_replay");

    let replay_state = {
        use futures::StreamExt;

        let mut source = gloo_net::eventsource::futures::EventSource::new("/api/events")
            .expect("couldn't connect to SSE stream");
        let s: ReadSignal<Option<Option<GameReplay>>> =
            create_signal_from_stream(source.subscribe("message").unwrap().map(
                |value| match value {
                    Ok(value) => {
                        let json_string = value.1.data().as_string().unwrap();
                        let val = serde_json::from_str(&json_string).unwrap();
                        Some(val)
                    }
                    Err(e) => None,
                },
            ));

        on_cleanup(move || source.close());
        s
    };

    let memo_state = create_memo(move |_c: Option<&GameState>| {
        let new_replay = replay_state.get().unwrap_or(None);
        if new_replay.is_none() {
            let seed = [0; 32];
            return GameState::new(&seed, 0);
        }
        let new_replay = new_replay.unwrap();
        match _c {
            None => {
                let seed = [0; 32];
                log::info!("memo none");
                GameState::new(&seed, 0)
            }
            Some(state_val) => {
                log::info!("memo Some");
                let mut state_val = state_val.clone();
                if (new_replay.replay_slices.len() == 0
                    || state_val.replay.replay_slices.len() == 0)
                    && (state_val.init_seed != new_replay.init_seed
                        || state_val.start_time != new_replay.start_time)
                {
                    log::info!("memo new thing");
                    state_val = GameState::new(&new_replay.init_seed, new_replay.start_time);
                } else {
                    log::info!("memo oold thing");
                    while state_val.replay.replay_slices.len() < new_replay.replay_slices.len() {
                        let idx = state_val.replay.replay_slices.len();
                        let pipi = new_replay.replay_slices[idx].clone();
                        if let Err(e) = state_val.accept_replay_slice(&pipi) {
                            log::warn!("error in accept_replay_slice() : {:?}", e);
                        }
                    }
                }
                state_val
            }
        }
    })
    .into_signal();

    // let count = create_sse_signal::<GameState>("game_state");

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    log::info!("sse demo");
    view! { <GameBoard on_reset_game=on_reset game_state=memo_state/> }
}
