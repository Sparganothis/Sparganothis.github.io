use crate::game::tet::{self, CellValue, GameReplaySegment, GameState};
use leptos::*;

const BOARD_HEIGHT: usize = 20;
///componenta

#[derive(Clone, PartialEq)]
pub struct BoardMatrixSignals {
    value: Vec<(usize, Vec<RwSignal<CellValue>>)>,
}

impl BoardMatrixSignals {
    pub fn new(val: Vec<(usize, Vec<CellValue>)>) -> Self {
        log::info!("NEW SIGGNALS ALL OVER AGAIN {} !!!!!1!!!!", val.len());
        Self {
            value: val
                .iter()
                .map(|x| (x.0, x.1.iter().map(|y| create_rw_signal(*y)).collect()))
                .collect(),
        }
    }

    pub fn update_value(
        _self: &Vec<(usize, Vec<RwSignal<CellValue>>)>,
        val: Vec<(usize, Vec<CellValue>)>,
    ) {
        for (r1, r2) in val.iter().zip(_self.iter()) {
            for (t1, t2) in r1.1.iter().zip(r2.1.iter()) {
                t2.update(move |xxx| {
                    if !xxx.eq(&t1) {
                        *xxx = *t1;
                        // log::info!("rewrite signal {:?}", *t1);
                    }
                });

                // t2.set(*t1);
            }
        }
    }
}

#[component]
pub fn BoardTable<const R: usize, const C: usize>(
    board: Signal<tet::BoardMatrix<R, C>>,
) -> impl IntoView {
    //
    // log::info!("redraw BoardTable R={} C={}", R, C);

    let (data, _set_data) = create_signal(
        BoardMatrixSignals::new({
            let mut v_new: Vec<_> = board
                .get_untracked()
                .rows()
                .into_iter()
                .enumerate()
                .collect();
            v_new.reverse();
            v_new
        })
        .value,
    );

    let do_update = move || {
        let board = {
            let mut v_new: Vec<_> = board.get().rows().into_iter().enumerate().collect();
            v_new.reverse();
            v_new
        };
        data.with(|data| {
            BoardMatrixSignals::update_value(data, board);
        });

        data.get()
    };

    // let signals = create_memo(
    //     move |_old: Option<&BoardMatrixSignals>| {
    //         if let Some(old_board) = _old {
    //             let mut old_board = old_board.clone();
    //             let board =
    //             v_new.reverse();
    //             old_board.update_value(v_new);

    //             old_board
    //         } else {
    //             log::info!("create  new signals!!");
    //             let board = board();

    //             let mut v_new: Vec<_> = board.rows().into_iter().enumerate().collect();
    //             v_new.reverse();

    //             BoardMatrixSignals::new(v_new)
    //         }
    //     },
    // );
    view! {
        <table cellpadding="0" cellspacing="0" border="0">
            <tbody>
                <For
                    each=do_update
                    key=|r| { r.0 }
                    children=|r| view! { <BoardRow row_vals=r.1 row_idx=r.0/> }
                />

            </tbody>
        </table>
    }
}

#[component]
pub fn BoardRow(row_vals: Vec<RwSignal<CellValue>>, row_idx: usize) -> impl IntoView {
    let iter = move || row_vals.clone().into_iter().enumerate();
    let overflow = row_idx >= BOARD_HEIGHT;

    view! {
        <tr>
            // <td>  {{row_idx}} </td>
            <For
                each=iter
                key=|c| c.0
                children=move |c| {
                    view! {
                        <td>
                            <BoardCell cell=c.1 overflow=overflow/>
                        </td>
                    }
                }
            />

        </tr>
    }
}

#[component]
pub fn BoardCell(cell: RwSignal<CellValue>, overflow: bool) -> impl IntoView {
    let lambda = move || {
        let _cell_cls = match cell.get() {
            tet::CellValue::Piece(p) => format!("tet {}", p.name()),
            tet::CellValue::Empty => "empty".to_string(),
            tet::CellValue::Garbage => "garbage".to_string(),
            tet::CellValue::Ghost => "ghost".to_string(),
        };
        let overflow_txt = if overflow { "overflow_cell" } else { "cell" };
        let _cell_cls = format!("{_cell_cls} {overflow_txt}");
        _cell_cls
    };

    view! { <div class=lambda></div> }
}

use super::style::*;
#[component]
pub fn GameBoard(
    #[prop(into)] game_state: RwSignal<tet::GameState>,
    on_reset_game: Callback<()>,
) -> impl IntoView {
    let tet_style = GameBoardTetStyle::new();
    let bottom_free_percent = 15.0;
    let cell_width_vmin = (100. - 2. * bottom_free_percent) / BOARD_HEIGHT as f64;

    // let _style = stylist::Style::new(style_str).expect("Failed to create style");
    let _style_name = default_style(tet_style, bottom_free_percent, cell_width_vmin)
        .get_class_name()
        .to_owned();

    let hold_board = create_read_slice(game_state, |state: &tet::GameState| state.get_hold_board());

    let next_board = create_read_slice(game_state, |state: &tet::GameState| state.get_next_board());

    let main_board = create_read_slice(game_state, |state: &tet::GameState| state.main_board);

    let gameover = view! {
        <Show when=move || game_state.get().game_over fallback=|| view! {}>
            <h3 style="color:red" on:click=move |_| on_reset_game.call(())>
                GAME OVER
            </h3>
        </Show>
    };

    let debug_info = move || game_state.get().get_debug_info();

    view! {
        <div class=_style_name>

            <div class="main_container">
                <div class="gameover">{gameover}</div>
                <div class="side_board_left">
                    <h3 class="side_board_title">HOLD</h3>

                    <BoardTable board=hold_board/>
                </div>

                <div class="score_window_left">
                    <h1 class="side_board_code">
                        {move || { format!("{:?}", game_state.get().score) }}
                    </h1>
                </div>

                <div class="main_board">
                    <BoardTable board=main_board/>
                </div>

                <div class="label_bottom">
                    <code class="side_board_code">{debug_info}</code>
                </div>

                <div class="side_board_right">
                    <h3 class="side_board_title">NEXT</h3>
                    <BoardTable board=next_board/>
                </div>
            // <div class="score_window_right">
            // <h3 class="side_board_title">{format!("{:?}", last_action.get())}</h3>
            // </div>
            </div>
        </div>
    }
}

use crate::game::random::GameSeed;
use crate::game::tet::TetAction;
use crate::game::timestamp::get_timestamp_now_nano;

pub fn key_debounce_ms(_action: TetAction) -> i64 {
    match _action {
        TetAction::HardDrop => 100,
        _ => 16,
    }
}

use crate::server::api::game_replay::GameId;

#[component]
pub fn PlayerGameBoard() -> impl IntoView {
    let new_game_id = create_resource(
        || (),
        |_| async move { crate::server::api::game_replay::create_new_game_id().await },
    );

    let on_state_change = Callback::<GameState>::new(move |s| {
        use crate::server::api::game_replay::append_game_segment;
        log::info!("we changed state: {}", s.get_debug_info());

        let game_id = new_game_id.get().unwrap().unwrap();

        let segment: GameReplaySegment = {
            if s.replay.replay_slices.is_empty() {
                GameReplaySegment::Init(s.replay)
            } else if s.game_over {
                GameReplaySegment::GameOver
            } else {
                GameReplaySegment::Update(s.replay.replay_slices.last().unwrap().clone())
            }
        };
        log::info!("segment: {:?}", &segment);
        spawn_local(async move {
            log::info!("spawn local ... ");
            if let Err(err) = append_game_segment(game_id, (&segment).clone()).await {
                log::warn!("fail to updload segmnet {:?} : {:?}", segment, err);
            }
            log::info!("spawn local OK!");
        });
    });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        // append_game_segment
        new_game_id.refetch();
    });
    let game_state = move || {
        if let Some(Ok(game_id)) = new_game_id.get() {
            view! {
                <PlayerGameBoardSingle game_id on_reset on_state_change/>
            }
            .into_view()
        } else {
            view! {
                <p> loading game id ... </p>
            }
            .into_view()
        }
    };

    view! {
         {game_state}
    }
}

#[component]
pub fn PlayerGameBoardSingle(
    game_id: GameId,
    on_reset: Callback<()>,
    on_state_change: Callback<GameState>,
) -> impl IntoView {
    let state = create_rw_signal(tet::GameState::new(&game_id.init_seed, game_id.start_time));
    on_state_change.call(state.get());

    let leptos_use::utils::Pausable {
        pause: _timer_pause,
        resume: _timer_resume,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            state.update(move |state| {
                if !state.game_over {
                    if state
                        .apply_action_if_works(TetAction::SoftDrop, get_timestamp_now_nano())
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

    let (get_ts, set_ts) = create_signal(std::collections::HashMap::<TetAction, i64>::new());
    let on_action: Callback<TetAction> = Callback::<TetAction>::new(move |_action| {
        let timestamp1 = crate::game::timestamp::get_timestamp_now_ms();
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

#[component]
pub fn RandomOpponentGameBoard(seed: GameSeed) -> impl IntoView {
    let state = create_rw_signal(tet::GameState::new(&seed, get_timestamp_now_nano()));
    let leptos_use::utils::Pausable {
        pause: _,
        resume: _,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            state.update(move |state| {
                let random_action = crate::game::tet::TetAction::random();
                let _ = state.apply_action_if_works(random_action, get_timestamp_now_nano());
            })
        },
        1000,
    );

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        if state.get().game_over {
            state.set(GameState::new(&seed, get_timestamp_now_nano()));
        }
    });

    view! { <GameBoard game_state=state on_reset_game=on_reset/> }
}
