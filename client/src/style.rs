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
pub fn default_style(
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
        height: 96.2vmin;
        width: ${cell_width_vmin * (2.0 * SIDE_BOARD_WIDTH as f64 + 11.8)}vmin;
        border: 0.2vmin solid green;
        position: absolute;
        top: 0vmin;
        left: 0vmin;
        overflow: hidden;
        z-index: -1;
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

).expect("bad css");
    st
}
