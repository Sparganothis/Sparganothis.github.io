use game::api::game_replay::{GameId, GameSegmentId};
use leptos::*;

use crate::websocket::demo_comp::WebsocketAPI;
use game::tet::{GameReplaySegment, GameState};


#[component]
pub fn SpectatorGameBoard(game_id: GameId) -> impl IntoView {
    let state = create_rw_signal(GameState::new(&game_id.init_seed, game_id.start_time));
    let api : WebsocketAPI = expect_context();
    let game_id = game_id.clone();
    let api2 = api.clone();
    let game_id2 = game_id.clone();
    api.subscribe_to_game(&game_id, Callback::<_>::new(move |_update: Vec<(GameSegmentId, GameReplaySegment)>| {
        state.update(|state_val| {
            for (key, _value) in _update.iter() {
                log::info!("got surpriuzxe segment: {}",key.segment_id);
                match _value {
                    GameReplaySegment::Init(init) => {
                            *state_val = GameState::new(&init.init_seed, init.start_time)
                    }
                    GameReplaySegment::Update(slice) => {
                            if let Err(e) = state_val.accept_replay_slice(&slice) {
                                log::warn!("error in accept_replay_slice() : {:?}", e);
                            }
                    }
                    GameReplaySegment::GameOver(_reason) => {
                        log::info!("subscribe game got over!");
                        state_val.game_over_reason = Some(_reason.clone());
                        let api2 = api2.clone();
                        queue_microtask(move || {
                            
                            log::info!("stop subscribe to game");
                            api2.stop_subscribe_to_game(&game_id2);
                        })
                    }
                }
            }
        });
    }));

    let api3 = api.clone();
    on_cleanup(move || {
        api3.stop_subscribe_to_game(&game_id2);
    });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {});
    view! {
        <GameBoardFlex
            on_reset_game=on_reset
            player_id=game_id.user_id
            game_state=state
            top_bar=view! { <h5>"spectating game"</h5> }.into_view()
        />
    }
}

use crate::comp::game_board_flex::GameBoardFlex;