use game::{bot::get_bot_id, random::GameSeed};
use game::tet::TetAction;
use game::timestamp::get_timestamp_now_nano;
use crate::comp::game_board_bot::BotGameBoardSingle;
use game::tet::{self, CellValue};
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
    #[prop(default = Callback::<(i8, i8)>::new(move |_| {}))]
    #[prop(optional)]
    on_click: Callback<(i8, i8)>,

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
            let mut v_new: Vec<_> =
                board.get().rows().into_iter().enumerate().collect();
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
        <table cellpadding="0" cellspacing="0" border="0" class="grid">
            <tbody>
                <For
                    each=do_update
                    key=|r| { r.0 }
                    children=move |r| {
                        let cb = {
                            Callback::<
                                i8,
                            >::new(move |_x| {
                                let y = r.0 as i8;
                                on_click.call((y, _x));
                            })
                        };
                        view! { <BoardRow row_vals=r.1 row_idx=r.0 on_click=cb/> }
                    }
                />

            </tbody>
        </table>
    }
}

#[component]
pub fn BoardRow(
    row_vals: Vec<RwSignal<CellValue>>,
    row_idx: usize,
    on_click: Callback<i8>,
) -> impl IntoView {
    let iter = move || row_vals.clone().into_iter().enumerate();
    let overflow = row_idx >= BOARD_HEIGHT;

    let tr_cls =( if overflow {"overflow_row"} else {""}).to_string();

    view! {
        <tr class=tr_cls>
            // <td>  {{row_idx}} </td>
            <For
                each=iter
                key=|c| c.0
                children=move |c| {
                    let cb = {
                        Callback::<
                            (),
                        >::new(move |_| {
                            let x = c.0;
                            on_click.call(x as i8);
                        })
                    };
                    view! {
                        <td>
                            <BoardCell cell=c.1 overflow=overflow on_click=cb/>
                        </td>
                    }
                }
            />

        </tr>
    }
}

#[component]
pub fn BoardCell(
    cell: RwSignal<CellValue>,
    overflow: bool,
    on_click: Callback<()>,
) -> impl IntoView {
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

    view! { <div class=lambda on:click=move |_| on_click.call(())></div> }
}




pub fn key_debounce_ms(_action: TetAction) -> i64 {
    match _action {
        TetAction::HardDrop => 100,
        _ => 16,
    }
}

#[component]
pub fn RandomOpponentGameBoard(seed: GameSeed) -> impl IntoView {
    let state = create_rw_signal(tet::GameState::new(&seed, get_timestamp_now_nano()));

    view! {
        <BotGameBoardSingle
            state=state
            bot_name="wordpress".to_string()
            bot_id=get_bot_id("wordpress").expect("bot exists")
        />
    }
}
