use anyhow::Context;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::timestamp::get_timestamp_now_nano;

use super::rot::{RotDirection, RotState, Shape};

use super::random::*;

use std::collections::VecDeque;
use std::time::Duration;
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord,
)]
pub enum Tet {
    I,
    L,
    J,
    T,
    S,
    Z,
    O,
}

impl Tet {
    pub fn spawn_pos(&self) -> (i8, i8) {
        const O_SPAWN_POS: (i8, i8) = (SPAWN_POS.0 + 1, SPAWN_POS.1 + 1);
        const I_SPAWN_POS: (i8, i8) = (SPAWN_POS.0 - 1, SPAWN_POS.1);
        match self {
            &Self::I => I_SPAWN_POS,
            &Self::O => O_SPAWN_POS,
            _ => SPAWN_POS,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            &Self::I => "I",
            &Self::L => "L",
            &Self::J => "J",
            &Self::T => "T",
            &Self::S => "S",
            &Self::Z => "Z",
            &Self::O => "O",
        }
    }

    pub fn shape(&self, rot_state: super::rot::RotState) -> Shape {
        let mut sh = self.orig_shape();
        match rot_state {
            super::rot::RotState::R0 => {}

            super::rot::RotState::R1 => {
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
            }

            super::rot::RotState::R2 => {
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
            }

            super::rot::RotState::R3 => {
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
                sh = super::rot::rotate_shape(sh, super::rot::RotDirection::Right);
            }
        }
        sh
    }

    pub fn orig_shape(&self) -> Shape {
        match self {
            &Self::I => vec![
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
            ],
            &Self::L => vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, true],
            ],
            &Self::J => vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![true, false, false],
            ],
            &Self::T => vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
            &Self::S => vec![
                vec![false, false, false],
                vec![true, true, false],
                vec![false, true, true],
            ],
            &Self::Z => vec![
                vec![false, false, false],
                vec![false, true, true],
                vec![true, true, false],
            ],
            &Self::O => vec![vec![true, true], vec![true, true]],
        }
    }
    pub fn random() -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let choices = Self::all();
        let mut rng = thread_rng();
        *choices.choose(&mut rng).unwrap()
    }
    pub fn all() -> Vec<Self> {
        vec![
            Self::I,
            Self::L,
            Self::J,
            Self::T,
            Self::S,
            Self::Z,
            Self::O,
        ]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellValue {
    // 4 bit after, before 16-20bit b
    Piece(Tet),
    Garbage,
    Empty,
    Ghost,
}
use serde_with::serde_as;
#[serde_as]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMatrix<const R: usize = 40, const C: usize = 10> {
    // 400 * cellValue = 1600bit after / 8000 before -- 200byte after, 1k before
    // with no color -- 400bit = 80bytes
    #[serde_as(as = "[[_; C]; R]")]
    vv: [[CellValue; C]; R],
}

impl<const R: usize, const C: usize> BoardMatrix<R, C> {
    pub fn inject_single_garbage_line(&mut self, seed: GameSeed) {
        let v: u8 = get_determinist_val::<u8>(&seed) % C as u8;

        // move all things up
        for i in (0..(R - 2)).rev() {
            for j in 0..C {
                let lower: CellValue = self.get_cell(i as i8, j as i8).unwrap();
                self.set_cell(i as i8 + 1, j as i8, lower);
            }
        }

        for x in 0..(C as i8) {
            if x != (v as i8) {
                self.set_cell(0, x, CellValue::Garbage)
            } else {
                self.set_cell(0, x, CellValue::Empty)
            }
        }
    }
    pub fn get_cell(&self, y: i8, x: i8) -> Option<CellValue> {
        if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
            None
        } else {
            Some(self.vv[y as usize][x as usize])
        }
    }

    pub fn set_cell(&mut self, y: i8, x: i8, v: CellValue) {
        self.vv[y as usize][x as usize] = v;
    }

    pub fn rows(&self) -> Vec<Vec<CellValue>> {
        self.vv
            .iter()
            .map(|r| r.iter().cloned().collect())
            .collect()
    }
    pub fn empty() -> Self {
        Self {
            vv: [[CellValue::Empty; C]; R],
        }
    }

    pub fn get_num_rows(&self) -> usize {
        R
    }
    pub fn get_num_cols(&self) -> usize {
        C
    }

    pub fn spawn_piece(&mut self, info: &CurrentPcsInfo) -> anyhow::Result<()> {
        let CurrentPcsInfo {
            pos: (y, x),
            tet: piece,
            rs: rot_state,
            id: _,
        } = *info;

        // if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
        //     );
        // }
        // let (x, y) = (x as usize, y as usize);
        let shape = piece.shape(rot_state);
        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if *cell {
                    let (cx, cy) = (x + i as i8, y + j as i8);
                    if cx < 0 || cy < 0 || cx >= (C as i8) || cy >= (R as i8) {
                        anyhow::bail!(
                    "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})");
                    }
                    match self.get_cell(cy, cx).unwrap() {
                        CellValue::Empty | CellValue::Ghost => {}
                        CellValue::Garbage | CellValue::Piece(_) => {
                            anyhow::bail!("cell position already taken");
                        }
                    }
                }
            }
        }

        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if *cell {
                    let (cx, cy) = (x + i as i8, y + j as i8);
                    if cx < 0 || cy < 0 || cx >= (C as i8) || cy >= (R as i8) {
                        anyhow::bail!(
                    "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})");
                    }
                    match self.get_cell(cy, cx).unwrap() {
                        CellValue::Empty | CellValue::Ghost => {
                            self.set_cell(cy, cx, CellValue::Piece(piece));
                        }
                        CellValue::Garbage | CellValue::Piece(_) => {
                            anyhow::bail!("cell position already taken");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn spawn_ghost(&mut self, info: &CurrentPcsInfo) -> anyhow::Result<()> {
        let CurrentPcsInfo {
            pos: (y, x),
            tet: piece,
            rs: rot_state,
            id: _,
        } = *info;

        let shape = piece.shape(rot_state);

        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if *cell {
                    let (cx, cy) = (x + i as i8, y + j as i8);
                    if cx < 0 || cy < 0 || cx >= (C as i8) || cy >= (R as i8) {
                        anyhow::bail!(
                    "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})");
                    }
                    match self.get_cell(cy, cx).unwrap() {
                        CellValue::Empty | CellValue::Ghost => {
                            self.set_cell(cy, cx, CellValue::Ghost);
                        }
                        CellValue::Garbage | CellValue::Piece(_) => {
                            anyhow::bail!("cell position already taken");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn delete_piece(&mut self, info: &CurrentPcsInfo) -> anyhow::Result<()> {
        let CurrentPcsInfo {
            pos: (y, x),
            tet: piece,
            rs: rot_state,
            id: _,
        } = *info;

        let shape = piece.shape(rot_state);
        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if *cell {
                    let (cx, cy) = (x + i as i8, y + j as i8);
                    if cx < 0 || cy < 0 || cx >= (C as i8) || cy >= (R as i8) {
                        anyhow::bail!(
                    "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})");
                    }
                    self.set_cell(cy, cx, CellValue::Empty);
                }
            }
        }
        Ok(())
    }
    pub fn spawn_nextpcs(&mut self, next_pcs: &VecDeque<Tet>) {
        let col: i8 = 0;
        let mut row: i8 = R as i8 - 4;
        for (i, piece) in next_pcs.iter().enumerate() {
            if i >= 5 {
                break;
            }
            let info = CurrentPcsInfo {
                id: 0,
                pos: (row + if (*piece).eq(&Tet::O) { 1 } else { 0 }, col),
                tet: *piece,
                rs: RotState::R0,
            };
            let r = self.spawn_piece(&info);
            row -= 3;
            if r.is_err() {
                log::info!("{r:?}");
            }
        }
    }

    pub fn get_height(&self) -> i32 {
        for i in (0..R).rev() {
            for j in 0..C {
                let cc = self.get_cell(i as i8, j as i8).unwrap();
                match cc {
                    CellValue::Piece(_) => return i as i32,
                    CellValue::Garbage => return i as i32,
                    CellValue::Empty => continue,
                    CellValue::Ghost => continue,
                };
            }
        }
        0
    }

    pub fn get_height_for_column(&self, col: i32) -> i32 {
        for x in (0..self.get_num_rows()).rev() {
            match self.get_cell(x as i8, col as i8).unwrap() {
                crate::tet::CellValue::Piece(_) => return x as i32,
                crate::tet::CellValue::Garbage => return x as i32,
                crate::tet::CellValue::Empty => continue,
                crate::tet::CellValue::Ghost => continue,
            }
        }
        0
    }

    pub fn board_holes(&self) -> i32 {
        let mut holes: i32 = 0;

        for x in (0..self.get_num_cols()).rev() {
            let height = self.get_height_for_column(x as i32);

            for y in 0..height {
                match self.get_cell(y as i8, x as i8).unwrap() {
                    crate::tet::CellValue::Empty | crate::tet::CellValue::Ghost => {
                        holes += 1;
                    }
                    _ => {}
                };
            }
        }

        holes
    }
    pub fn board_bumpi(&self) -> i32 {
        let mut max_bumpi = 0;
        for i in 0..(self.get_num_cols() - 1) {
            let left = i;
            let right = i + 1;
            let height_left = self.get_height_for_column(left as i32);
            let height_right = self.get_height_for_column(right as i32);

            let bumpi = height_left - height_right;
            let bumpi = if bumpi > 0 { bumpi } else { -bumpi };
            if bumpi > max_bumpi {
                max_bumpi = bumpi;
            }
        }
        max_bumpi
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TetAction {
    // 3bit (8 acctions)
    HardDrop,
    SoftDrop,
    MoveLeft,
    MoveRight,
    Hold,
    RotateLeft,
    RotateRight,
    Nothing,
}

impl TetAction {
    pub fn name(&self) -> String {
        format!("{self:?}")
    }
    pub fn all() -> Vec<TetAction> {
        vec![
            Self::HardDrop,
            Self::SoftDrop,
            Self::MoveLeft,
            Self::MoveRight,
            Self::Hold,
            Self::RotateLeft,
            Self::RotateRight,
        ]
    }
    pub fn is_repeating(&self) -> bool {
        match self {
            TetAction::MoveLeft | TetAction::MoveRight | TetAction::SoftDrop => true,
            _ => false,
        }
    }
    pub fn random() -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        if thread_rng().gen_bool(0.5) {
            Self::SoftDrop
        } else {
            let choices = [
                Self::HardDrop,
                Self::SoftDrop,
                Self::MoveLeft,
                Self::MoveRight,
                Self::Hold,
                Self::RotateLeft,
                Self::RotateRight,
            ];
            let mut rng = thread_rng();
            *choices.choose(&mut rng).unwrap()
        }
    }
}

pub const SIDE_BOARD_WIDTH: usize = 4;
type BoardMatrixHold = BoardMatrix<3, SIDE_BOARD_WIDTH>;
type BoardMatrixNext = BoardMatrix<16, SIDE_BOARD_WIDTH>;
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub score: i32,              // 24 bits
    pub is_t_spin: bool,         // 1 bit
    pub is_t_mini_spin: bool,    // 1 bit
    pub is_b2b: bool,            // 1 bit
    pub combo_counter: i8,       // 7 bit
    pub main_board: BoardMatrix, // ?? bit
    // pub next_board: BoardMatrixNext,
    // pub hold_board: BoardMatrixHold,
    pub last_action: TetAction,              // 3 bit
    pub next_pcs: VecDeque<Tet>,             // 42 bit
    pub current_pcs: Option<CurrentPcsInfo>, // 29 bit
    pub current_id: u16,                     // 13 bit

    pub hold_pcs: Option<HoldPcsInfo>, // 4 bit
    // pub game_over: bool,
    pub game_over_reason: Option<GameOverReason>, // 3 bit

    pub replay: GameReplay,  // OK
    pub seed: GameSeed,      // 32 bytes = 256bit
    pub init_seed: GameSeed, // 256bit
    pub start_time: i64,     // n--ai acsf
    pub total_lines: u16,
    pub total_garbage_sent: u16, // 15 bit
    pub garbage_recv: u16,       // 15 bit
    pub garbage_applied: u16,
    pub total_moves: u16, // 16 bit
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplay {
    pub init_seed: GameSeed,
    pub start_time: i64,
    pub replay_slices: Vec<GameReplaySlice>,
}

impl GameReplay {
    pub fn empty(seed: &GameSeed, start_time: i64) -> Self {
        Self {
            init_seed: *seed,
            start_time,
            replay_slices: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameOverReason {
    Knockout,
    Disconnect,
    Abandon,
    Win,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameReplaySegment {
    Init(GameReplay),
    Update(GameReplaySlice),
    GameOver(GameOverReason),
}

// impl GameReplaySegment {
//     pub fn is_game_over(&self) -> bool {
//         match self {
//             Self::Update(slice) => slice.event.game_over,
//             _ => false,
//         }
//     }
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplaySlice {
    pub idx: u16,
    pub event: GameReplayEvent,
    pub event_timestamp: i64,
    pub new_seed: GameSeed,
    pub new_garbage_recv: u16,
    pub new_garbage_applied: u16,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayEvent {
    pub action: TetAction,
    // pub game_over: bool,
}

const SPAWN_POS: (i8, i8) = (18, 3);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldPcsInfo {
    pub can_use: bool,
    pub tet: Tet,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentPcsInfo {
    // total 29bit, with no bitfield = 64bit
    pub pos: (i8, i8), // max 40 , max 10 --> 6bit + 4bit = 10bit
    pub tet: Tet,      // 3bit
    pub rs: RotState,  // 2bit
    pub id: u16,       // 14bit
}

impl GameState {
    pub fn add_pending_garbage(&mut self) {
        while self.garbage_applied < self.garbage_recv {
            self.main_board.inject_single_garbage_line(self.seed);
            self.garbage_applied += 1;
        }
    }
    pub fn apply_raw_garbage(&mut self, new_garbage: u16) {
        if new_garbage > self.garbage_recv {
            self.garbage_recv = new_garbage;
        }
    }
    pub fn current_time_string(&self) -> String {
        let dt_s = self.current_time_sec();
        if dt_s < 0 {
            return "future".to_string();
        }
        let duration = Duration::from_secs(dt_s as u64);
        format!("{:?}", duration)
    }
    pub fn current_time_sec(&self) -> i64 {
        let now = get_timestamp_now_nano();
        let dt_nano = now - self.start_time;
        let dt_s = dt_nano / 1000000;
        dt_s
    }

    pub fn game_over(&self) -> bool {
        self.game_over_reason.is_some()
    }

    pub fn new(seed: &GameSeed, start_time: i64) -> Self {
        let mut new_state = Self {
            score: 0,
            combo_counter: -1,
            is_t_spin: false,
            is_t_mini_spin: false,
            is_b2b: false,
            main_board: BoardMatrix::empty(),
            // next_board: BoardMatrixNext::empty(),
            // hold_board: BoardMatrixHold::empty(),
            last_action: TetAction::Nothing,
            next_pcs: VecDeque::new(),
            current_pcs: None,
            game_over_reason: None,
            hold_pcs: None,
            current_id: 0,
            seed: *seed,
            init_seed: *seed,
            replay: GameReplay::empty(seed, start_time),
            start_time,
            total_lines: 0,
            total_garbage_sent: 0,
            garbage_recv: 0,
            total_moves: 0,
            garbage_applied: 0,
        };
        new_state.refill_nextpcs(start_time);
        let _ = new_state.put_next_piece(start_time);
        new_state.put_ghost();
        new_state
    }

    pub fn empty() -> Self {
        let seed = [0; 32];
        let start_time = 0;
        Self::new(&seed, start_time)
    }
    pub fn get_debug_info(&self) -> String {
        format!(
            "total_lines:{}\n total_garbage_sent:{}\n garbage_apply:{}\n garbage_rcv:{}",
            self.total_lines, self.total_garbage_sent, self.garbage_applied, self.garbage_recv
        )
    }

    fn clear_line(&mut self) {
        let mut lines = 0;
        while let Some(line) = self.can_clear_line() {
            for i in line..39 {
                for j in 0..10 {
                    let upper = self.main_board.get_cell(i + 1, j).unwrap();
                    self.main_board.set_cell(i, j, upper);
                }
            }
            lines += 1;
        }
        self.add_score_for_clear_line(lines);
        self.total_lines += lines;
    }

    fn add_score_for_clear_line(&mut self, lines: u16) {
        let mut score = 0;
        let mut score2 = 0;
        let mut score3 = 0;
        score += match lines {
            1 => 40,
            2 => 80,
            3 => 160,
            4 => 320,
            _ => 0,
        };

        if self.is_gameboard_empty() {
            score2 += match lines {
                1 => 200,
                2 => 400,
                3 => 800,
                4 => 1600,
                _ => 0,
            };
        }
        if self.is_t_spin {
            score3 += match lines {
                1 => 1000,
                2 => 2000,
                3 => 3000,
                _ => 0,
            };
        }
        if self.is_t_mini_spin {
            score3 += match lines {
                1 => 666,
                2 => 1666,
                3 => 2666,
                _ => 0,
            };
        }
        self.is_b2b = (lines == 4) || (self.is_t_spin);
        self.score += (score + score2 + score3) as i32;
        self.is_t_spin = false;
        self.is_t_mini_spin = false;
        if lines > 0 {
            self.combo_counter += 1;
        } else {
            self.combo_counter = -1;
        }
        if self.combo_counter > 0 {
            self.score += 50 * self.combo_counter as i32;
        }
        self.total_garbage_sent += match self.combo_counter {
            1 | 2 => 1,
            3 | 4 => 2,
            5 | 6 => 3,
            _i if _i >= 7 => 4,
            _ => 0,
        };
    }

    fn can_clear_line(&self) -> Option<i8> {
        for (i, row) in self.main_board.rows().into_iter().enumerate() {
            // let row = self.main_board.v[i];
            let is_full = row
                .iter()
                .map(|cell| match cell {
                    CellValue::Piece(_) => true,
                    CellValue::Garbage => true,
                    CellValue::Empty => false,
                    CellValue::Ghost => false,
                })
                .reduce(|a, b| a & b);
            if let Some(is_full) = is_full {
                if is_full {
                    return Some(i as i8);
                }
            }
        }

        None
    }
    fn put_replay_event(&mut self, event: &GameReplayEvent, event_time: i64) {
        let idx = self.replay.replay_slices.len() as u16;
        let new_seed = accept_event(&self.seed, event, event_time, idx as u32);
        let new_slice = GameReplaySlice {
            idx,
            event: event.clone(),
            new_seed,
            event_timestamp: event_time,
            new_garbage_recv: self.garbage_recv,
            new_garbage_applied: self.garbage_applied,
        };
        self.seed = new_slice.new_seed;
        // log::info!("put  replay event {new_slice:?}");
        self.replay.replay_slices.push(new_slice);
    }

    fn refill_nextpcs(&mut self, event_time: i64) {
        while self.next_pcs.len() < 6 {
            // log::info!("next refill");
            let (new_pcs2, new_seed) = shuffle_tets(&self.seed, event_time);
            for n in new_pcs2 {
                self.next_pcs.push_back(n);
            }
            self.seed = new_seed;
        }
    }
    fn put_next_piece(&mut self, _event_time: i64) -> anyhow::Result<()> {
        if self.current_pcs.is_some() {
            log::warn!("cannont put next pcs because we already have one");
            anyhow::bail!("already have next pcs");
        }

        if self.game_over() {
            log::warn!("game over but you called put_next_cs");
            anyhow::bail!("game already over");
        }

        self.clear_line();
        self.add_pending_garbage();
        let next_tet = self.next_pcs.pop_front().unwrap();

        self.current_pcs = Some(CurrentPcsInfo {
            pos: next_tet.spawn_pos(),
            tet: next_tet,
            id: self.current_id,
            rs: RotState::R0,
        });
        self.current_id += 1;

        if let Err(_) = self.main_board.spawn_piece(&self.current_pcs.unwrap()) {
            log::info!("tet game over");
            self.game_over_reason = Some(GameOverReason::Knockout);
        } else if let Some(ref mut h) = self.hold_pcs {
            h.can_use = true;
        }
        Ok(())
    }

    pub fn accept_replay_slice(
        &mut self,
        slice: &GameReplaySlice,
    ) -> anyhow::Result<()> {
        // log::info!("over={} acccept replay slice: {:?}", self.game_over, slice);
        if let Some(prev_slice) = self.replay.replay_slices.last() {
            if slice.idx != prev_slice.idx + 1 {
                anyhow::bail!("duplicate slice mismatch");
            }
        } else {
            if slice.idx != 0 {
                anyhow::bail!(
                    "first slice mismatch: got slice {} expected slice {}",
                    slice.idx,
                    0
                );
            }
        }
        // TODO FIGURE OUT BEFORE OR AFTER
        self.garbage_recv = slice.new_garbage_recv;
        *self = self.try_action(slice.event.action, slice.event_timestamp)?;
        let self_slicce = self.replay.replay_slices.last().unwrap();
        if !slice.eq(self_slicce) {
            log::warn!(
                "no  match in last slicec:  recieved == {:?},  rebuildt locally == ={:?}",
                slice,
                self_slicce
            )
        }
        Ok(())
    }
    pub fn get_next_board(&self) -> BoardMatrixNext {
        let mut b = BoardMatrixNext::empty();
        b.spawn_nextpcs(&self.next_pcs);
        b
    }

    pub fn get_hold_board(&self) -> BoardMatrixHold {
        let mut b = BoardMatrixHold::empty();
        if let Some(HoldPcsInfo { can_use: _, tet }) = self.hold_pcs {
            let info = CurrentPcsInfo {
                tet,
                pos: (if tet.eq(&Tet::I) { -1 } else { 0 }, 0),
                rs: RotState::R0,
                id: 0,
            };
            if let Err(e) = b.spawn_piece(&info) {
                log::warn!("hold board cannot spawn piece WTF: {:?}", e);
            }
        }
        b
    }

    fn try_hold(&mut self, event_time: i64) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        let old_hold = self.hold_pcs.clone();
        if let Some(ref old_hold) = old_hold {
            if !old_hold.can_use {
                anyhow::bail!("can_use=false for hold");
            }
        }

        self.hold_pcs = Some(HoldPcsInfo {
            tet: current_pcs.tet,
            can_use: false,
        });

        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }
        self.current_pcs = None;

        if let Some(ref old_hold) = old_hold {
            self.next_pcs.push_front(old_hold.tet);
        }
        self.put_next_piece(event_time)?;
        self.hold_pcs = Some(HoldPcsInfo {
            tet: current_pcs.tet,
            can_use: false,
        });

        Ok(())
    }

    fn try_harddrop(&mut self, event_time: i64) -> anyhow::Result<()> {
        let mut soft_drops: i16 = 0;
        let current_pcs = self.current_pcs.context("no current pcs")?;
        let mut r = self.try_softdrop(event_time);
        while r.is_ok() && current_pcs.id == self.current_pcs.unwrap().id {
            r = self.try_softdrop(event_time);
            soft_drops += 1;
        }
        self.score += 10;
        self.score -= (soft_drops * 2) as i32;
        Ok(())
    }

    fn try_softdrop(&mut self, event_time: i64) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }
        let mut new_current_pcs = current_pcs;
        new_current_pcs.pos.0 -= 1;
        if self.main_board.spawn_piece(&new_current_pcs).is_ok() {
            self.score += 2;
            self.current_pcs = Some(new_current_pcs);
            self.is_t_spin = false;
            self.is_t_mini_spin = false;
        } else {
            self.main_board.spawn_piece(&current_pcs).unwrap();
            self.current_pcs = None;
            self.put_next_piece(event_time)?;
        }
        Ok(())
    }

    fn try_moveleft(&mut self) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let mut new_current_pcs = current_pcs;
        new_current_pcs.pos.1 -= 1;

        self.main_board.spawn_piece(&new_current_pcs)?;
        self.current_pcs = Some(new_current_pcs);
        Ok(())
    }

    fn try_moveright(&mut self) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let mut new_current_pcs = current_pcs;
        new_current_pcs.pos.1 += 1;

        self.main_board.spawn_piece(&new_current_pcs)?;
        self.current_pcs = Some(new_current_pcs);
        Ok(())
    }

    fn try_rotate(&mut self, rot: RotDirection) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;
        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let before = &current_pcs.rs;
        let after = &current_pcs.rs.rotate(rot);

        for (_try_idx, (x, y)) in
            super::rot::srs_offsets(*before, *after, *(&current_pcs.tet))
                .iter()
                .enumerate()
        {
            let mut new_current_pcs: CurrentPcsInfo = current_pcs;
            new_current_pcs.rs = *after;
            // warning! table above in (x, y) but our repr in (y, x)
            new_current_pcs.pos.0 += y;
            new_current_pcs.pos.1 += x;

            if let Ok(_) = self.main_board.spawn_piece(&new_current_pcs) {
                let (t_is_blocked3, t_is_blocked2) = {
                    let (yt, xt) = new_current_pcs.pos;
                    let mut block_counter = 0;
                    for (dx, dy) in [(0, 0), (0, 2), (2, 0), (2, 2)] {
                        let px = dx + xt;
                        let py = dy + yt;
                        block_counter += match self.main_board.get_cell(py, px) {
                            Some(CellValue::Piece(_)) => 1,
                            Some(CellValue::Garbage) => 1,
                            Some(CellValue::Empty) => 0,
                            Some(CellValue::Ghost) => 0,
                            None => 0,
                        };
                    }
                    (block_counter >= 3, block_counter >= 2)
                };
                self.current_pcs = Some(new_current_pcs);
                self.is_t_spin = if new_current_pcs.tet == Tet::T {
                    t_is_blocked3
                } else {
                    (*x != 0) || (*y != 0)
                };
                self.is_t_mini_spin = new_current_pcs.tet == Tet::T && t_is_blocked2;
                return Ok(());
            }
        }

        anyhow::bail!("all ooffset are blocked")
    }

    pub fn try_action(
        &self,
        action: TetAction,
        event_time: i64,
    ) -> anyhow::Result<Self> {
        if self.game_over() {
            // log::warn!("gamem over cannot try_action");
            anyhow::bail!("game over");
        }
        let mut new = self.clone();
        new.last_action = action;
        new.refill_nextpcs(event_time);

        match action {
            TetAction::HardDrop => {
                new.try_harddrop(event_time)?;
            }
            TetAction::SoftDrop => {
                new.try_softdrop(event_time)?;
            }
            TetAction::MoveLeft => {
                new.try_moveleft()?;
            }
            TetAction::MoveRight => {
                new.try_moveright()?;
            }
            TetAction::Hold => {
                new.try_hold(event_time)?;
            }
            TetAction::RotateLeft => {
                new.try_rotate(RotDirection::Left)?;
            }
            TetAction::RotateRight => {
                new.try_rotate(RotDirection::Right)?;
            }
            TetAction::Nothing => {}
        }
        let ev = GameReplayEvent {
            action,
            // game_over: self.game_over,
        };
        new.put_replay_event(&ev, event_time);
        new.clear_ghost();
        if !new.game_over() {
            new.put_ghost();
        }
        new.total_moves += 1;
        Ok(new)
    }

    fn put_ghost(&mut self) {
        let mut ghost_board = self.main_board.clone();
        let info = self.current_pcs.unwrap();
        ghost_board
            .delete_piece(&info)
            .expect("cannot delete pice in put_ghost");

        let mut final_ghost_board = None;

        for y in (-3..info.pos.0).rev() {
            let mut ghost_info = info.clone();
            ghost_info.pos.0 = y;
            if ghost_board.spawn_piece(&ghost_info).is_err() {
                ghost_info.pos.0 += 1;
                final_ghost_board = Some(ghost_info);
                break;
            } else {
                if ghost_board.delete_piece(&ghost_info).is_err() {
                    log::warn!("cannot delete temporary ghost");
                }
            }
        }

        if let Some(ghost_info) = final_ghost_board {
            let _ = self.main_board.spawn_ghost(&ghost_info);
        }
    }

    fn is_gameboard_empty(&mut self) -> bool {
        let mut gameboard_empty = true;
        for y in 0..self.main_board.get_num_rows() {
            for x in 0..self.main_board.get_num_cols() {
                let value = self.main_board.get_cell(y as i8, x as i8).unwrap();
                // let value = (&self.main_board.v)[y][x];
                if !(value.eq(&CellValue::Ghost) || value.eq(&CellValue::Empty)) {
                    gameboard_empty = false;
                }
            }
        }
        gameboard_empty
    }

    fn clear_ghost(&mut self) {
        for y in 0..self.main_board.get_num_rows() {
            for x in 0..self.main_board.get_num_cols() {
                let old_value = self.main_board.get_cell(y as i8, x as i8).unwrap();
                if old_value.eq(&CellValue::Ghost) {
                    self.main_board.set_cell(y as i8, x as i8, CellValue::Empty);
                }
            }
        }
    }
    pub fn apply_action_if_works(
        &mut self,
        action: TetAction,
        event_time: i64,
    ) -> anyhow::Result<()> {
        let r = self.try_action(action, event_time);
        if let Ok(new_state) = r {
            *self = new_state;
            Ok(())
        } else {
            let e = r.unwrap_err();
            // log::warn!("user action {:?} failed: {:?}", action, e);
            Err(e)
        }
    }
}

pub fn segments_to_states(all_segments: &Vec<GameReplaySegment>) -> Vec<GameState> {
    let mut current_state = match all_segments.get(0) {
        Some(GameReplaySegment::Init(_replay)) => {
            GameState::new(&_replay.init_seed, _replay.start_time)
        }
        _ => {
            log::info!("got no init segment");
            return vec![];
        }
    };
    let mut all_states = vec![];
    all_states.push(current_state.clone());
    for segment in &all_segments[1..] {
        match segment {
            GameReplaySegment::Init(_) => {
                log::error!("got two init segments");
                return vec![];
            }
            GameReplaySegment::Update(_slice) => {
                if let Err(e) = current_state.accept_replay_slice(_slice) {
                    log::error!("failed to accept replay slice: {:#?}", e);
                    return vec![];
                }
            }
            GameReplaySegment::GameOver(reason) => {
                current_state.game_over_reason = Some(reason.clone());
            }
        }
        all_states.push(current_state.clone());
    }
    all_states
}

#[cfg(test)]
pub mod tests {
    use super::super::timestamp::get_timestamp_now_nano;
    use super::*;
    // use pretty_assertions::assert_eq;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    pub fn random_have_pinned_results() {
        let seed = [0; 32];
        let mut state = GameState::new(&seed, 0);

        // let expected_seed = [0;32];
        // assert_eq!(expected_seed, state.seed);

        state.apply_action_if_works(TetAction::SoftDrop, 0).unwrap();

        let expected_seed = [
            171, 40, 80, 152, 75, 128, 86, 158, 75, 125, 16, 157, 144, 245, 5, 162,
            114, 11, 172, 187, 117, 160, 51, 219, 154, 112, 95, 249, 135, 175, 135,
            202,
        ];
        assert_eq!(expected_seed, state.seed);

        state.apply_action_if_works(TetAction::HardDrop, 1).unwrap();

        let expected_seed = [
            191, 181, 134, 68, 198, 122, 193, 86, 133, 117, 213, 55, 160, 168, 100, 54,
            183, 31, 91, 168, 236, 182, 197, 72, 247, 194, 248, 34, 211, 234, 107, 43,
        ];
        assert_eq!(expected_seed, state.seed);
    }

    #[test]
    #[wasm_bindgen_test]
    pub fn active_game_is_deterministic() {
        for i in 0..255 {
            let seed = [i; 32];
            let mut state1 = GameState::new(&seed, get_timestamp_now_nano());
            let mut state2 = GameState::new(&seed, state1.start_time);

            loop {
                let action = TetAction::random();
                let t2 = get_timestamp_now_nano();
                let res1 = state1.try_action(action, t2).map_err(|_| "bad");
                let res2 = state2.try_action(action, t2).map_err(|_| "bad");
                assert_eq!(res1, res2);
                if res1.is_ok() {
                    state1 = res1.unwrap();
                    state2 = res2.unwrap();
                }

                if state1.game_over() {
                    break;
                }
            }
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn passive_game_tracks_active_one() {
        for i in 0..255 {
            let seed = [i; 32];

            let mut active_game = GameState::new(&seed, get_timestamp_now_nano());
            let mut passive_game = GameState::new(&seed, active_game.start_time);
            let mut _slices = vec![];

            loop {
                let action = TetAction::random();
                let res = active_game.try_action(action, get_timestamp_now_nano());
                if let Ok(new_active_game) = res {
                    active_game = new_active_game;
                }
                if active_game.game_over() {
                    _slices = active_game.replay.replay_slices;
                    break;
                }
            }

            for slice in _slices {
                passive_game.accept_replay_slice(&slice).unwrap();
            }

            // assert_eq!(active_game, passive_game);
        }
    }
}
