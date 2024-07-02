use std::collections::VecDeque;

use crate::comp::game_board_player::PRE_123_INTERVAL;
use crate::websocket::demo_comp::call_api_sync;
use game::api::{game_replay::GameId, websocket::*};
use game::timestamp::get_timestamp_now_nano;
use leptos_use::{ use_interval_with_options, UseIntervalOptions, UseIntervalReturn};
use game::tet::{self, GameReplaySegment, GameState};
use leptos::*;
use rand::Rng;
use crate::comp::game_board_flex::GameBoardFlex;

pub const BOT_MOVE_INTERVAL: u64 = 84;

#[component]
pub fn BotGameBoard(
    game_id: GameId,
    bot_name: String,
) -> impl IntoView {
     let on_state_change = Callback::<GameState>::new(move |s| {
        let segment: GameReplaySegment = {
            if s.replay.replay_slices.is_empty() {
                GameReplaySegment::Init(s.replay)
            } else if s.game_over {
                GameReplaySegment::GameOver(tet::GameOverReason::Knockout)
            } else {
                GameReplaySegment::Update(
                    s.replay.replay_slices.last().unwrap().clone(),
                )
            }
        };

        let segment_json: String = serde_json::to_string(&segment).unwrap();
        call_api_sync::<AppendBotGameSegment>((game_id, segment_json), move |_r| {
            // log::info!("append OK: {:?}", _r);
        });
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
    
    let state = create_rw_signal(
        tet::GameState::new(&game_id.init_seed, game_id.start_time));

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

            <BotGameBoardSingle
                state
                on_state_change
                bot_id=game_id.user_id
                bot_name=bot_name.clone()
            />
        </Show>
    }
}



#[component]
pub fn BotGameBoardSingle(
    state: RwSignal<GameState>,

    #[prop(default = Callback::<GameState>::new(move |_| {}))]
    #[prop(optional)]
    on_state_change: Callback<GameState>,

    bot_id: uuid::Uuid,
    bot_name: String,

) -> impl IntoView {

    on_state_change.call(state.get_untracked());
    let extra_moves = create_rw_signal(VecDeque::new()); 

    let bot_name2 = bot_name.clone();
    let current_time_in_6 = create_rw_signal(0);
    let leptos_use::utils::Pausable {
        pause: _timer_pause,
        resume: _timer_resume,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            let bot_name2 = bot_name2.clone();
            let _t6 = current_time_in_6.get_untracked();
            current_time_in_6.set_untracked((current_time_in_6.get_untracked() + 1) % 6);

            let r20percent =(&mut rand::thread_rng()).gen_bool(0.20);
            if (!(_t6 == 0 || _t6 == 3 || _t6 == 5)) || r20percent {
                return;
            }

            state.update(move |state| {
                if !state.game_over {

                    if extra_moves.get_untracked().is_empty() {
                        let bot = game::bot::get_bot(&bot_name2).unwrap();
                        if let Ok(action) =  bot.as_ref().choose_move(state) {
                            extra_moves.update_untracked(move |extra_moves| {
                                for act in action {
                                    extra_moves.push_back(act);
                                }
                            });
                        }
                    }

                    extra_moves.update_untracked(move |extra_moves| {
                        if let Some(the_move) = extra_moves.pop_front() {
                            if state                            .apply_action_if_works(
                                the_move,
                                get_timestamp_now_nano(),
                            )
                                .is_ok()
                            {
                                on_state_change.call(state.clone());
                            }
                        } else {
                            if state                            .apply_action_if_works(
                                tet::TetAction::SoftDrop,
                                get_timestamp_now_nano(),
                            )
                                .is_ok()
                            {
                                on_state_change.call(state.clone());
                            }
                        }
                    });
                }
            });
        },
        BOT_MOVE_INTERVAL,
    );

    let top_bar = view! { "active bot" }.into_view();

    view! { <GameBoardFlex game_state=state top_bar enable_sound=true player_id=bot_id/> }
}