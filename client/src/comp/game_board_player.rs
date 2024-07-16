use crate::comp::hotkey_reader::create_hotkey_reader;
use crate::mobile_check::is_mobile_phone;
use crate::{comp::game_board::key_debounce_ms, websocket::demo_comp::call_api_sync};
use game::api::{game_replay::GameId, websocket::*};
use game::tet::TetAction;
use game::timestamp::get_timestamp_now_nano;
use leptos_use::{ use_interval_with_options, UseIntervalOptions, UseIntervalReturn};
use game::tet::{self, GameState};
use leptos::*;
use crate::comp::game_board_flex::GameBoardFlex;
use game::tet::GameReplaySegment;
pub const PRE_123_INTERVAL: u64 = 400;
pub const AUTO_SOFTDROP_INTERVAL: u64 = 1000;

#[component]
pub fn PlayerGameBoardFromId(
    #[prop(optional)]
    #[prop(default = Callback::<GameState, GameState>::new(move |x| x))]
    control_callback: Callback<GameState, GameState>,

    game_id: GameId,
    #[prop(default = Callback::<()>::new(move |_| {}))]
    #[prop(optional)]
    on_reset: Callback<()>,
) -> impl IntoView {
    
    let is_mobile = is_mobile_phone();
    if is_mobile {
        return view! { <h1>You are phone. <br/> Plz use PC.</h1> }.into_view()
    }
    
    let state: RwSignal<GameState> = create_rw_signal(
        tet::GameState::new(&game_id.init_seed, game_id.start_time));

    let send_to_server = Callback::<GameReplaySegment>::new(move |segment| {
        log::info!("player sending to server: {:?}", segment);
        let segment_json: String = serde_json::to_string(&segment).expect("serialize segmment ot json");
        call_api_sync::<AppendGameSegment>((game_id, segment_json), move |_r| {
            if let Some(gamme_over_reasoon) = _r.maybe_reason {
                state.update(|state| {
                    log::info!("player game over because {:?}", gamme_over_reasoon)    ;    
                    state.game_over_reason = Some(gamme_over_reasoon.clone());
                    state.last_segment = GameReplaySegment::GameOver(gamme_over_reasoon.clone());
                })
            }
            
            if _r.garbage > 0 {
                state.update(|state| {
                    state.apply_raw_garbage(_r.garbage);
                } )
            }
        });
    });
    
    let on_state_change = Callback::<GameState>::new(move |s| {
        send_to_server.call(s.last_segment.clone());
        let s2 = control_callback.call(s.clone());
        if s2.last_segment != s.last_segment {
            send_to_server.call(s2.last_segment);
        }
    });

    let UseIntervalReturn {
        counter: counter_pre_123,
        pause: pause_pre_123,
        resume: resume_pre_123,
        ..
    }  = use_interval_with_options( PRE_123_INTERVAL, UseIntervalOptions::default().immediate(false) );
        
    let (pre_countdown_text, set_countdown_text) = create_signal("".to_string());
    create_effect(move |_| {
        let counter_val = counter_pre_123.get();
        let new = match counter_val {
            0 => "3".to_string(),
            1 => "2".to_string(),
            2 => "1".to_string(),
            3 => "Go".to_string(),
            _ => "".to_string(),
        };
        set_countdown_text.set(new);
        if counter_val > 5 {
            pause_pre_123();
        }
    });
    

    call_api_sync::<SetGlobalPlayLock>((true, Some(game_id)), move |_| {
        let resume_pre_123 = resume_pre_123.clone();
        call_api_sync::<GetLastFullGameState>(game_id, move |_state| {
            match _state {
                Some(_state) => {
                    state.set(_state);
                    resume_pre_123();
                },
                None => {
                    resume_pre_123();
                }
            }
        });
    });

    on_cleanup(move || {
        call_api_sync::<SetGlobalPlayLock>((false, None), move |_| {});
    });
    
    view! {
        <Show
            when=move || { counter_pre_123.get() > 3 }
            fallback=move || {
                view! {
                    <GameBoardFlex
                        game_state=state
                        pre_countdown_text
                        enable_sound=true
                        player_id=game_id.user_id
                    />
                }
            }
        >

            <PlayerGameBoardSingle
                state
                on_reset
                on_state_change
                player_id=game_id.user_id
            />
        </Show>
    }.into_view()
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

    
    #[prop(into)]
    #[prop(default = create_signal("".to_string()).0)]
    #[prop(optional)]
    pre_countdown_text: ReadSignal<String>,
    

    #[prop(into)]
    #[prop(default = view!{}.into_view())]
    top_bar: View,

    player_id: uuid::Uuid,

) -> impl IntoView {

    on_state_change.call(state.get_untracked());

    let leptos_use::utils::Pausable {
        pause: _timer_pause,
        resume: _timer_resume,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            state.update(move |state| {
                if !state.game_over() {
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
        AUTO_SOFTDROP_INTERVAL,
    );

    let reset_timer = move || {
        _timer_pause();
        _timer_resume();
    };

    let (get_ts, set_ts) =
        create_signal(std::collections::HashMap::<TetAction, i64>::new());
    create_hotkey_reader( move |_action| {
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
        <GameBoardFlex
            game_state=state
            on_reset_game=on_reset
            pre_countdown_text
            top_bar
            enable_sound=true
            player_id
        />
    }
}