use crate::game::tet::{self, CellValue, GameState, SIDE_BOARD_WIDTH};
use leptos::*;

const BOARD_HEIGHT: usize = 20;
///componenta
#[component]
pub fn BoardTable<const R: usize, const C: usize>(
    board: Signal<tet::BoardMatrix<R, C>>,
) -> impl IntoView {
    //
    // log::info!("redraw BoardTable R={} C={}", R, C);
    let values = move || {
        let board = board();
        let mut v: Vec<_> = board.rows().into_iter().enumerate().collect();
        v.reverse();
        v
    };
    view! {
        <table cellpadding="0" cellspacing="0" border="0">
            <tbody>
                <For
                    each=values
                    key=|r| { r.0 }
                    children=|r| view! { <BoardRow row_vals=r.1 row_idx=r.0/> }
                />

            </tbody>
        </table>
    }
}

#[component]
pub fn BoardRow(row_vals: Vec<CellValue>, row_idx: usize) -> impl IntoView {
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
pub fn BoardCell(cell: tet::CellValue, overflow: bool) -> impl IntoView {
    let _cell_cls = match cell {
        tet::CellValue::Piece(p) => format!("tet {}", p.name()),
        tet::CellValue::Empty => "empty".to_string(),
        tet::CellValue::Garbage => "garbage".to_string(),
    };
    let overflow_txt = if overflow { "overflow_cell" } else { "cell" };
    let _cell_cls = format!("{_cell_cls} {overflow_txt}");
    view! { <div class=_cell_cls></div> }
}

#[component]
pub fn GameBoard(
    #[prop(into)] game_state: Signal<tet::GameState>,
    on_reset_game: Callback<()>,
) -> impl IntoView {
    let bottom_free_percent = 15.0;
    let cell_width_vmin = (100. - 2. * bottom_free_percent) / BOARD_HEIGHT as f64;

    let default_style = stylist::style!(
        table {
            border-collapse: collapse;
        }
        .main_container {
            height: 97.9vmin;
            width: ${cell_width_vmin * (2.0 * SIDE_BOARD_WIDTH as f64 + 12.)}vmin;
            border: 0.2vmin solid green;
            position: absolute;
            top: 0vmin;
            left: 0vmin;
        }
        .main_board, .side_board_left, .side_board_right, .score_window_left, .score_window_right, .label_bottom {
            width: max-content;
            position: absolute;
            // border: 0.2vmin solid black;
            // padding: 0.2vmin; margin: 0.2vmin;
        }
        .side_board_title {
            height: ${cell_width_vmin}vmin;
            margin: 0px;
            text-align: center;
            line-height: ${cell_width_vmin}vmin;
            font-size: ${cell_width_vmin}vmin;
        }
        .side_board_code {
        }
        .side_board_left {
            left: ${cell_width_vmin/4.0}vmin;
            top: ${bottom_free_percent * 0.83}vmin;
        }
        .score_window_left {
            left: ${cell_width_vmin/4.0}vmin;
            top: ${bottom_free_percent * 0.83 +cell_width_vmin * (5.5 + SIDE_BOARD_WIDTH as f64)}vmin;
            width: ${cell_width_vmin * (0.0125 + SIDE_BOARD_WIDTH as f64)}vmin;
            height:  ${cell_width_vmin * (0.9 + 2. * SIDE_BOARD_WIDTH as f64)}vmin;
        }
        .side_board_right {
            left: ${cell_width_vmin * (SIDE_BOARD_WIDTH as f64 + 11.4)}vmin;
            top: ${bottom_free_percent * 0.83}vmin;
        }
        // .score_window_right {
        //     left: ${cell_width_vmin * (SIDE_BOARD_WIDTH as f64 + 11.4)}vmin;
        //     top: ${bottom_free_percent * 0.83 +cell_width_vmin * (12. + SIDE_BOARD_WIDTH as f64)}vmin;
        //     width: ${cell_width_vmin * (0.01 + SIDE_BOARD_WIDTH as f64)}vmin;
        //     height:  ${cell_width_vmin * (-0.5 + SIDE_BOARD_WIDTH as f64)}vmin;
        // }
        .label_bottom {
            left: ${cell_width_vmin*0.25}vmin;
            top: ${bottom_free_percent * 0.83 +cell_width_vmin * 22.}vmin;
            width: ${cell_width_vmin * (11.2 + 2. * SIDE_BOARD_WIDTH as f64)}vmin;
            height:  ${cell_width_vmin * 2.0}vmin;
        }

        .main_board {
            left: ${cell_width_vmin * (SIDE_BOARD_WIDTH as f64 + 0.8)}vmin;
            top: ${-cell_width_vmin*20. + bottom_free_percent * 0.83}vmin;
        }
        td {
            padding: 0px; margin: 0 px;
            border-collapse: collapse;
            height: ${cell_width_vmin}vmin;
            width: ${cell_width_vmin}vmin;
            max-height: ${cell_width_vmin}vmin;
            max-width: ${cell_width_vmin}vmin;
        }
        tr {border-collapse: collapse;padding: 0px; margin: 0 px;}

        .cell, .overflow_cell {
            height: 100%;
            width: 100%;
            display: block;
            font-size: 0px; line-height: 0px;
        }

        .cell {
            border: 1px gray solid;
        }
        .cell.empty {
            background-color: black;
        }

        .overflow_cell {
            border-color: 1px transparent;
            background-color: transparent;
        }
        .tet.S.cell {            background-color: lightgreen;     }
        .tet.T.cell {            background-color: magenta;     }
        .tet.I.cell {            background-color: lightblue;     }
        .tet.J.cell {            background-color: #48bef7;     }
        .tet.L.cell {            background-color: orange;     }
        .tet.O.cell {            background-color: yellow;     }
        .tet.Z.cell {            background-color: red;     }

    ).expect("bad css");

    // let _style = stylist::Style::new(style_str).expect("Failed to create style");
    let _style_name = default_style.get_class_name().to_owned();

    let hold_board = create_memo(move |_| game_state().get_hold_board()).into_signal();
    let hold_board = move || {
        view! { <BoardTable board=hold_board/> }
    };

    let next_board =
        create_memo(move |_| game_state.with(|game_state| game_state.get_next_board()))
            .into_signal();
    let next_board = move || {
        view! { <BoardTable board=next_board/> }
    };

    let main_board = create_memo(move |_| game_state().main_board).into_signal();
    let main_board = move || {
        view! { <BoardTable board=main_board/> }
    };

    let gameover = create_memo(move |_| {
        view! {
            <Show when=move || game_state().game_over fallback=|| view! {}>
                <h3 style="color:red" on:click=move |_| on_reset_game(())>
                    GAME OVER
                </h3>
            </Show>
        }
    });

    let debug_info = create_memo(move |_| game_state().get_debug_info()).into_signal();

    let gameboard_view = move || {
        view! {
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
        }
    };

    view! { <div class=_style_name>{move || gameboard_view()}</div> }
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
