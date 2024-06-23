use crate::{comp::game_board::{key_debounce_ms, GameBoard}, websocket::demo_comp::call_websocket_api};
use game::api::websocket::*;
use game::random::GameSeed;
use game::tet::TetAction;
use game::timestamp::get_timestamp_now_nano;
use crate::style::*;
use crate::websocket::demo_comp::WebsocketAPI;
use game::tet::{self, CellValue, GameReplaySegment, GameState};
use leptos::*;


#[component]
pub fn PlayerGameBoard() -> impl IntoView {
    let api = expect_context::<WebsocketAPI>();
    let api2 = api.clone();

    let new_game_id = create_resource(
        || (),
        move |_| {
            let api = api.clone();

            async move {
                let r = call_websocket_api::<CreateNewGameId>(api.clone(), ())
                    .expect("cannot obtain future")
                    .await;
                r.unwrap()
            }
        },
    );

    let on_state_change = Callback::<GameState>::new(move |s| {
        // log::info!("we changed state: {}", s.get_debug_info());

        let game_id = new_game_id.get().unwrap();

        let segment: GameReplaySegment = {
            if s.replay.replay_slices.is_empty() {
                GameReplaySegment::Init(s.replay)
            } else if s.game_over {
                GameReplaySegment::GameOver
            } else {
                GameReplaySegment::Update(
                    s.replay.replay_slices.last().unwrap().clone(),
                )
            }
        };
        // log::info!("segment: {:?}", &segment);
        spawn_local({
            let api2 = api2.clone();

            async move {
                // log::info!("calling websocket api");
                let segment_json: String =
                    serde_json::to_string(&segment).expect("json never fail");
                let _r = call_websocket_api::<AppendGameSegment>(
                    api2.clone(),
                    (game_id, segment_json),
                )
                .expect("cannot obtain future")
                .await;
                if let Err(e) = _r {
                    log::warn!("failed to append game segment: {}", e);
                }
                // log::info!("got back response: {:?}", r);
            }
        });
    });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        // append_game_segment
        new_game_id.refetch();
    });
    let game_state = move || {
        if let Some(game_id) = new_game_id.get() {
            let state =
            create_rw_signal(tet::GameState::new(&game_id.init_seed, game_id.start_time));
            view! { <PlayerGameBoardSingle state on_reset on_state_change/> }
                .into_view()
        } else {
            view! { <p>loading game id ...</p> }.into_view()
        }
    };

    view! { {game_state} }
}

#[component]
pub fn PlayerGameBoardSingle(
    state: RwSignal<GameState>,

    #[prop(default = Callback::<()>::new(move |_| {}))]
    #[prop(optional)]
    on_reset: Callback<()>,

    #[prop(default = Callback::<GameState>::new(move |_| {}))]
    #[prop(optional)]
    on_state_change: Callback<GameState>,
    
) -> impl IntoView {

    on_state_change.call(state.get_untracked());

    let leptos_use::utils::Pausable {
        pause: _timer_pause,
        resume: _timer_resume,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            state.update(move |state| {
                if !state.game_over {
                    if state
                        .apply_action_if_works(
                            TetAction::SoftDrop,
                            get_timestamp_now_nano(),
                        )
                        .is_ok()
                    {
                        on_state_change.call(state.clone());
                    }
                }
            })
        },
        1000,
    );

    let reset_timer = move || {
        _timer_pause();
        _timer_resume();
    };

    let (get_ts, set_ts) =
        create_signal(std::collections::HashMap::<TetAction, i64>::new());
    let on_action: Callback<TetAction> = Callback::<TetAction>::new(move |_action| {
        let timestamp1 = game::timestamp::get_timestamp_now_ms();
        let timestamp0 = *get_ts.get().get(&_action).unwrap_or(&0);
        if (timestamp1 - timestamp0) > key_debounce_ms(_action) {
            set_ts.update(move |m| {
                m.insert(_action, timestamp1);
            });
            state.update(|state| {
                if state
                    .apply_action_if_works(_action, get_timestamp_now_nano())
                    .is_ok()
                {
                    on_state_change.call(state.clone());
                    reset_timer();
                }
            })
        }
    });

    view! {
        <super::hotkey_reader::HotkeyReader on_action=on_action></super::hotkey_reader::HotkeyReader>
        <GameBoard game_state=state on_reset_game=on_reset/>
    }
}