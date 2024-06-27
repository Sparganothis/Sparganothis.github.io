use crate::{comp::game_board::key_debounce_ms, websocket::demo_comp::call_api_sync};
use game::api::{game_replay::GameId, websocket::*};
use game::tet::TetAction;
use game::timestamp::get_timestamp_now_nano;
use leptos_use::{use_interval, UseIntervalReturn};
use game::tet::{self, GameReplaySegment, GameState};
use leptos::*;


#[component]
pub fn PlayerGameBoardFromId(game_id: GameId) -> impl IntoView {
    let on_state_change = Callback::<GameState>::new(move |s| {

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

        let segment_json: String = serde_json::to_string(&segment).unwrap();
        call_api_sync::<AppendGameSegment>((game_id, segment_json), move |_r| {
            // log::info!("append OK: {:?}", _r);
        });
    });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        // append_game_segment
    });
    let UseIntervalReturn {
        counter,
        reset,
        // is_active,
        pause,
        resume,
        ..
    }  = use_interval( 1000 );
    pause();
    reset();
        
    let (pre_countdown_text, set_countdown_text) = create_signal("".to_string());
    
    create_effect(move |_| {
        let counter_val = counter.get();
        let new = match counter_val {
            0 => "3".to_string(),
            1 => "2".to_string(),
            2 => "1".to_string(),
            3 => "Go".to_string(),
            _ => "".to_string(),

        };
        set_countdown_text.set(new);
        if counter_val > 5 {
            pause();
        }
    });
    
    let game_id = game_id;
    let state = create_rw_signal(
        tet::GameState::new(&game_id.init_seed, game_id.start_time));

    reset();
    resume();
       
    view! {
        <Show
            when=move || { counter.get() > 3 }
            fallback=move || {
                view! { <GameBoardFlex game_state=state pre_countdown_text/> }
            }
        >

            <PlayerGameBoardSingle state on_reset on_state_change/>
        </Show>
    }
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
                log::info!("press action: {:?}", _action);
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
        <GameBoardFlex
            game_state=state
            on_reset_game=on_reset
            pre_countdown_text
            top_bar
        />
    }
}
use crate::comp::game_board_flex::GameBoardFlex;