

use leptos::*;
use leptos_router::*;
use leptris_frontend::tet::{self, CellValue};

const BOARD_HEIGHT: usize = 20;
const SIDE_BOARD_WIDTH: usize = 5;

#[component]
pub fn BoardTable<const R: usize, const C: usize>(board: tet::Board<R,C>) -> impl IntoView {
    let values = move || {
        let mut v: Vec<_> = board.rows().into_iter().enumerate().collect();
        v.reverse();
        v
    };
    view! {
        <table cellpadding="0" cellspacing="0"  border="0">
            <tbody>
                <For
                    each=values
                    key=|r| {r.0}
                    children=|r| view!{
                        <BoardRow overflow={r.0>=BOARD_HEIGHT} row_vals=r.1 row_idx={r.0} />
                    }
                />
            </tbody>
        </table>
    }
}

#[component]
pub fn BoardRow(overflow: bool, row_vals: Vec<CellValue>, row_idx: usize) -> impl IntoView {
    let iter = move || {
        row_vals.clone().into_iter().enumerate()
    };

    view!{
        <tr>
            // <td>  {{row_idx}} </td>
            <For
                each=iter
                key=|c| c.0
                children = move |c| {
                    view! {
                        <td>
                            <BoardCell cell=c.1 overflow=overflow />
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
    let overflow_txt = if overflow {"overflow_cell"} else {"cell"};
    let _cell_cls = format!("{_cell_cls} {overflow_txt}");
    view!{
        <div class=_cell_cls>
            // {{format!("{cell:?}")}}
        </div>
    }
}

#[component]
pub fn GameBoard() -> impl IntoView {

    let bottom_free_percent = 15.0;
    let cell_width_vmin = (100. - 2.*bottom_free_percent) / BOARD_HEIGHT as f64;

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
        .score_window_right {
            left: ${cell_width_vmin * (SIDE_BOARD_WIDTH as f64 + 11.4)}vmin;
            top: ${bottom_free_percent * 0.83 +cell_width_vmin * (12. + SIDE_BOARD_WIDTH as f64)}vmin;
            width: ${cell_width_vmin * (0.01 + SIDE_BOARD_WIDTH as f64)}vmin;
            height:  ${cell_width_vmin * (-0.5 + SIDE_BOARD_WIDTH as f64)}vmin;
        }
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
            background-color: black;
        }

        .overflow_cell {
            border-color: 1px transparent;
            background-color: transparent;
        }
    ).expect("bad css");

    // let _style = stylist::Style::new(style_str).expect("Failed to create style");
    let _style_name = default_style.get_class_name().to_owned();

    let main_board: tet::Board =  tet::Board::empty();
    let next_board = tet::Board::<14, SIDE_BOARD_WIDTH>::empty();
    let hold_board = tet::Board::<4, SIDE_BOARD_WIDTH>::empty();


    view! { 
        // class={{_style.get_class_name()}},

        <div class={{_style_name}}>
            <div class="main_container">
                <div class="side_board_left">
                    <h3 class="side_board_title">HOLD</h3>
                    <BoardTable board=hold_board/>
                </div>
                <div class="score_window_left">
                    <h3 class="side_board_title">PENIS</h3>
                </div>

                <div class="main_board">
                    <BoardTable board=main_board/>
                </div>
                <div class="label_bottom">
                    <h3 class="side_board_title">PENIS</h3>
                </div>

                <div class="side_board_right">
                    <h3 class="side_board_title">NEXT</h3>
                    <BoardTable board=next_board/>
                </div>
                <div class="score_window_right">
                    <h3 class="side_board_title">PENIS</h3>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn MainMenu() -> impl IntoView {
    let menu_entries = || {vec![
        ("/", "home"),
        ("/vs_cpu", "1v1 cpu"),
        ("/vs_net", "1v1 online"),
        ("/account", "account"),
        ("/settings", "settings"),
        ("/about", "about"),
        ("/credits", "credits"),
    ]};
    view!{
        <ul class="menu_root">
            <For 
                each=menu_entries
                key= |k| k.0
                children= |k| view!  {
                    <A href=k.0>
                    <h3 class="menu_item">
                    {k.1}
                    </h3>
                    </A>
                }
            />
        </ul>
    }
}

#[component]
pub fn AppRoot () -> impl IntoView {
    let _style = stylist::style!(
        nav {
            position: absolute;
            left: 0vmin;
            top: 0vmin;
            height: 98vmin;
            width: 18vmin;
            border: 1vmin solid black;
        }
        main {
            position: absolute;
            top: 0vmin;
            left: 19.85vmin;
            height: 100vmin;
        }
        main > div.main_left {
            position: absolute;
            top: 0vmin;
            width: 77vmin;
            height: 98vmin;
            border: 1vmin solid green;
        }
        main > div.main_right {
            position: absolute;
            top: 0vmin;
            width: 77vmin;
            left: 78.85vmin;
            height: 98vmin;
            border: 1vmin solid blue;
        }
        .menu_root {
            padding: 0px;
        }
        .menu_item {
            margin: 0px;
            height: 6vmin;
            text-align: center;
            line-height: 6vmin;
            font-size: 3vmin;
            font-weight: normal;
            color: black;
            rotate: -11deg;
        }
        a {
            text-decoration: none;
        }
        a[aria-current="page"] > .menu_item  {
            font-weight: bold;
            color: darkred;
            border: 0.5vmin darkred solid;
            margin: 0.5vmin;
            height: 5vmin;
            line-height: 5vmin;
        }
    ).expect("bad css");
    view! {
        <div class=_style.get_class_name().to_string()>
            <Router>
                <nav>
                    <MainMenu />
                </nav>
                <main>
                    // all our routes will appear inside <main>
                    <Routes>
                        <Route path="" view=|| {
                            view!{
                                <div class="main_left">
                                    <GameBoard/>
                                </div>
                            }
                        }> </Route>

                        <Route path="/vs_cpu" view=|| {
                            view!{
                                <div class="main_left">
                                    <GameBoard/>
                                </div>
                                <div class="main_right">
                                    <GameBoard/>
                                </div>
                            }
                        }> </Route>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}


fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { 
        <AppRoot />
    })
}