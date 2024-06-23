pub struct GameBoardTetStyle {
    pub s: String,
    pub z: String,
    pub t: String,
    pub o: String,
    pub i: String,
    pub j: String,
    pub l: String,
}

impl GameBoardTetStyle {
    pub fn new() -> Self {
        GameBoardTetStyle {
            s: "#74C21D".to_string(),
            z: "#FF4A58".to_string(),
            t: "#DA5DB2".to_string(),
            o: "#FFC125".to_string(),
            i: "#21B6F8".to_string(),
            j: "#4169E7".to_string(),
            l: "#FF8720".to_string(),
        }
    }
}
use game::tet::SIDE_BOARD_WIDTH;
pub fn default_gameboard_style(
    tet_style: GameBoardTetStyle,
    bottom_free_percent: f64,
    cell_width_vmin: f64,
) -> stylist::Style {
    #[allow(non_upper_case_globals)]
    let st = stylist::style!(
    table {
        border-collapse: collapse;
    }

    .main_container {
        height: 95.2vmin;
        width: ${cell_width_vmin * (2.0 * SIDE_BOARD_WIDTH as f64 + 11.8)}vmin;
        border: 0.2vmin solid green;
        position: absolute;
        top: 0vmin;
        left: 0vmin;
        overflow: hidden;
        // z-index: -1;
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
    .cell.ghost {
        background-color: #555;
    }

    .overflow_cell {
        border-color: 1px transparent;
        background-color: transparent;
    }
    .tet.S.cell {            background-color: ${tet_style.s};     }
    .tet.T.cell {            background-color: ${tet_style.t};    }
    .tet.I.cell {            background-color: ${tet_style.i};     }
    .tet.J.cell {            background-color: ${tet_style.j};     }
    .tet.L.cell {            background-color: ${tet_style.l};     }
    .tet.O.cell {            background-color: ${tet_style.o};     }
    .tet.Z.cell {            background-color: ${tet_style.z};     }


    .game_over_display {
        color: #f00c;
        font-weight: bold;
        top: 28vmin;
        left: 16vmin;
        width: 40vmin;
        height: 40vmin;
        font-size: 12vmin;
        text-align: center;
        z-index: 888;
        position: absolute;
        background-color: #000c;
        animation: spin 5s linear 1;
        border: 1vh #f00c;
        font-family: "Comic Sans MS", "Comic Sans", cursive;
    }

).expect("bad css");
    st
}


use leptos_struct_table::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-300",
            template_classes
        )
    }

    fn thead_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800"
            }
        } else if selected {
            "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
        } else {
            "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
        };

        format!(
            "{} {} {}",
            "border-b dark:border-gray-700", bg_color, template_classes
        )
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("{} {}", "px-5 py-2", prop_class)
    }

    fn loading_cell_inner(&self, row_index: usize, _col_index: usize, prop_class: &str) -> String {
        let width = match row_index % 4 {
            0 => "w-[calc(85%-2.5rem)]",
            1 => "w-[calc(90%-2.5rem)]",
            2 => "w-[calc(75%-2.5rem)]",
            _ => "w-[calc(60%-2.5rem)]",
        };
        format!(
            "animate-pulse h-2 bg-gray-200 rounded-full dark:bg-gray-700 inline-block align-middle {} {}",
            width, prop_class
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}