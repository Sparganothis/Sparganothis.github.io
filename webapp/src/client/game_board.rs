use crate::game::tet::{self, CellValue, GameState, SIDE_BOARD_WIDTH};
use leptos::*;

const BOARD_HEIGHT: usize = 20;
///componenta

#[derive(Clone, PartialEq)]
pub struct BoardMatrixSignals {
    value: Vec<(usize, Vec<RwSignal<CellValue>>)>,
}

impl BoardMatrixSignals {
    pub fn new(val: Vec<(usize, Vec<CellValue>)>) -> Self {
        log::info!("NEW SIGGNALS ALL OVER AGAIN !!!!!1!!!!");
        Self {
            value: val
                .iter()
                .map(|x| (x.0, x.1.iter().map(|y| create_rw_signal(*y)).collect()))
                .collect(),
        }
    }

    pub fn update_value(&mut self, val: Vec<(usize, Vec<CellValue>)>) {
        for (r1, r2) in val.iter().zip(self.value.iter()) {
            for (t1, t2) in r1.1.iter().zip(r2.1.iter()) {
                t2.update(move |xxx| {
                    if !xxx.eq(&t1) {
                        *xxx = *t1;
                        log::info!("rewrite signal {:?}", *t1);
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

    let board_val = {
        let mut v_new: Vec<_> = board().rows().into_iter().enumerate().collect();
        v_new.reverse();
        v_new
    };
    let signals = BoardMatrixSignals::new(board_val);



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
                    each=move || {signals.value.clone()}
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
    #[prop(into)] game_state: Signal<tet::GameState>,
    on_reset_game: Callback<()>,
) -> impl IntoView {
    let tet_style = GameBoardTetStyle::new();
    let bottom_free_percent = 15.0;
    let cell_width_vmin = (100. - 2. * bottom_free_percent) / BOARD_HEIGHT as f64;

    // let _style = stylist::Style::new(style_str).expect("Failed to create style");
    let _style_name = default_style(tet_style, bottom_free_percent, cell_width_vmin)
        .get_class_name()
        .to_owned();

    let hold_board = create_memo(move |_| game_state().get_hold_board()).into_signal();
    let hold_board =     view! { <BoardTable board=hold_board/> }    ;

    let next_board =
        create_memo(move |_| game_state.with(|game_state| game_state.get_next_board()))
            .into_signal();
    let next_board = view! { <BoardTable board=next_board/> };

    let main_board = create_memo(move |_| game_state().main_board).into_signal();
    let main_board = view! { <BoardTable board=main_board/> };

    let gameover = view! {
            <Show when=move || game_state().game_over fallback=|| view! {}>
                <h3 style="color:red" on:click=move |_| on_reset_game(())>
                    GAME OVER
                </h3>
            </Show>
        };

    let debug_info = create_memo(move |_| game_state().get_debug_info()).into_signal();

    let gameboard_view =  view! {
            <div class="main_container">
                <div class="gameover">{gameover}</div>
                <div class="side_board_left">
                    <h3 class="side_board_title">HOLD</h3>
                    {hold_board}
                </div>

                <div class="score_window_left">
                    <code class="side_board_code">
                        {move || { format!("{:?}", game_state().score) }}
                    </code>
                </div>

                <div class="main_board">{main_board}</div>
                <div class="label_bottom">
                    <code class="side_board_code">{debug_info.get()}</code>
                </div>

                <div class="side_board_right">
                    <h3 class="side_board_title">NEXT</h3>
                    {next_board}
                </div>
            // <div class="score_window_right">
            // <h3 class="side_board_title">{format!("{:?}", last_action.get())}</h3>
            // </div>
            </div>
        };

    view! { <div class=_style_name>{gameboard_view}</div> }
}

use crate::game::random::GameSeed;
use crate::game::tet::TetAction;
use crate::game::timestamp::get_timestamp_now;

#[component]
pub fn PlayerGameBoard(seed: GameSeed) -> impl IntoView {
    let (get_state, _set_state) = create_signal(tet::GameState::new(&seed, get_timestamp_now()));

    let leptos_use::utils::Pausable {
        pause: _timer_pause,
        resume: _timer_resume,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            _set_state.update(move |state| {
                let _ = state.apply_action_if_works(TetAction::SoftDrop, get_timestamp_now());
            })
        },
        1000,
    );

    let reset_timer = move || {
        _timer_pause();
        _timer_resume();
    };

    let on_action: Callback<TetAction> = Callback::<TetAction>::new(move |_action| {
        _set_state.update(|state| {
            if state
                .apply_action_if_works(_action, get_timestamp_now())
                .is_ok()
            {
                reset_timer();
            }
        })
    });

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        if get_state().game_over {
            _set_state.set(GameState::new(&seed, get_timestamp_now()));
        }
    });

    view! {
        <super::hotkey_reader::HotkeyReader on_action=on_action></super::hotkey_reader::HotkeyReader>
        <GameBoard game_state=get_state on_reset_game=on_reset/>
    }
}

#[component]
pub fn OpponentGameBoard(seed: GameSeed) -> impl IntoView {
    let (get_state, _set_state) = create_signal(tet::GameState::new(&seed, get_timestamp_now()));
    let leptos_use::utils::Pausable {
        pause: _,
        resume: _,
        is_active: _,
    } = leptos_use::use_interval_fn(
        move || {
            _set_state.update(move |state| {
                let random_action = crate::game::tet::TetAction::random();
                let _ = state.apply_action_if_works(random_action, get_timestamp_now());
            })
        },
        1000,
    );

    let on_reset: Callback<()> = Callback::<()>::new(move |_| {
        if get_state().game_over {
            _set_state.set(GameState::new(&seed, get_timestamp_now()));
        }
    });

    view! { <GameBoard game_state=get_state on_reset_game=on_reset/> }
}
