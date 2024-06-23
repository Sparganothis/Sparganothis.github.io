use game::api::game_replay::{GameId, GameSegmentId};
use game::api::websocket::{SubscribeGamePlz, SubscribeGamePlzArgument};
use leptos::*;

use crate::comp::game_board::GameBoard;
use crate::websocket::demo_comp::{call_websocket_api, WebsocketAPI};
use game::random::GameSeed;
use game::tet::{GameReplaySegment, GameState};

#[component]
pub fn SpectatorGameBoard(game_id: GameId) -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let state = create_rw_signal(GameState::new(&seed, 0));
    let api : WebsocketAPI = expect_context();
    let game_id = game_id.clone();
    api.subscribe_to_game(&game_id, Callback::<_>::new(move |_update: Vec<(GameSegmentId, GameReplaySegment)>| {
        for (key, _value) in _update.iter() {
            log::info!("got surpriuzxe segment: {}",key.segment_id);
            match _value {
                GameReplaySegment::Init(init) => {
                    state.update(|state_val| {
                        *state_val = GameState::new(&init.init_seed, init.start_time)
                    });
                }
                GameReplaySegment::Update(slice) => {
                    state.update(|state_val| {
                        if let Err(e) = state_val.accept_replay_slice(&slice) {
                            log::warn!("error in accept_replay_slice() : {:?}", e);
                        }
                    });
                }
                GameReplaySegment::GameOver => {
                    log::info!("subscribe game over!");
                }
            }
        }
    }));

    // let memo_state = create_memo(move |_| {
    //     let segment = get_segment();
    //     match segment {

    //         None => {
    //             log::info!("websocket message is none!~");
    //             false
    //         }
    //     }
    // });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    view! {
        <h1>"soectate demo"</h1>
        <GameBoard on_reset_game=on_reset game_state=state/>
    }
}
