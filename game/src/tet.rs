use anyhow::Context;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::rot::{RotDirection, RotState, Shape};

use super::random::*;

use std::collections::VecDeque;
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
    Piece(Tet),
    Garbage,
    Empty,
    Ghost,
}
use serde_with::serde_as;
#[serde_as]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMatrix<const R: usize = 40, const C: usize = 10> {
    #[serde_as(as = "[[_; C]; R]")]
    pub v: [[CellValue; C]; R],
}

impl<const R: usize, const C: usize> BoardMatrix<R, C> {
    pub fn get_height(&self) -> i32 {
        for i in (0..R).rev() {
            for j in 0..C {
                let cc = self.v[i][j];
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
    pub fn get_num_rows(&self) -> usize {
        R
    }
    pub fn get_num_cols(&self) -> usize {
        C
    }

    pub fn get_cell(&self, y: i8, x: i8) -> Option<CellValue> {
        if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
            None
        } else {
            Some(self.v[y as usize][x as usize])
        }
    }

    pub fn empty() -> Self {
        Self {
            v: [[CellValue::Empty; C]; R],
        }
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
                    match self.v[cy as usize][cx as usize] {
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
                    match self.v[cy as usize][cx as usize] {
                        CellValue::Empty | CellValue::Ghost => {
                            self.v[cy as usize][cx as usize] = CellValue::Piece(piece);
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
                    match self.v[cy as usize][cx as usize] {
                        CellValue::Empty | CellValue::Ghost => {
                            self.v[cy as usize][cx as usize] = CellValue::Ghost;
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
                    self.v[cy as usize][cx as usize] = CellValue::Empty;
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
    pub fn rows(&self) -> Vec<Vec<CellValue>> {
        self.v.iter().map(|r| r.iter().cloned().collect()).collect()
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TetAction {
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
    pub score: i64,
    pub is_t_spin: bool,
    pub is_t_mini_spin: bool,
    pub have_combo: bool,
    pub main_board: BoardMatrix,
    // pub next_board: BoardMatrixNext,
    // pub hold_board: BoardMatrixHold,
    pub last_action: TetAction,
    pub next_pcs: VecDeque<Tet>,
    pub current_pcs: Option<CurrentPcsInfo>,
    pub current_id: u32,

    pub hold_pcs: Option<HoldPcsInfo>,
    pub game_over: bool,

    pub replay: GameReplay,
    pub seed: GameSeed,
    pub init_seed: GameSeed,
    pub start_time: i64,
    pub total_lines: i64,
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    pub idx: u32,
    pub event: GameReplayEvent,
    pub event_timestamp: i64,
    pub new_seed: GameSeed,
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
    pub pos: (i8, i8),
    pub tet: Tet,
    pub rs: RotState,
    pub id: u32,
}

impl GameState {
    pub fn new(seed: &GameSeed, start_time: i64) -> Self {
        let mut new_state = Self {
            score: 0,
            have_combo: false,
            is_t_spin: false,
            is_t_mini_spin: false,
            main_board: BoardMatrix::empty(),
            // next_board: BoardMatrixNext::empty(),
            // hold_board: BoardMatrixHold::empty(),
            last_action: TetAction::Nothing,
            next_pcs: VecDeque::new(),
            current_pcs: None,
            game_over: false,
            hold_pcs: None,
            current_id: 0,
            seed: *seed,
            init_seed: *seed,
            replay: GameReplay::empty(seed, start_time),
            start_time,
            total_lines: 0,
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
        format!("is_t_spin:{}", self.is_t_spin)
    }

    fn clear_line(&mut self) {
        let mut score = 0;
        let mut lines = 0;
        let mut score2 = 0;
        let mut score3 = 0;

        while let Some(line) = self.can_clear_line() {
            for i in line..39 {
                for j in 0..10 {
                    self.main_board.v[i as usize][j] =
                        self.main_board.v[i as usize + 1][j];
                }
            }
            lines += 1;
            self.have_combo = true;
        }
        if self.have_combo {
            self.score += 50;
            self.have_combo = false;
        }
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
            self.is_t_spin = false;
        }
        if self.is_t_mini_spin {
            score3 += match lines {
                1 => 666,
                2 => 1666,
                3 => 2666,
                _ => 0,
            };
            self.is_t_mini_spin = false;
        }

        self.score += (score + score2 + score3) as i64;
        self.total_lines += lines;
    }

    fn can_clear_line(&self) -> Option<i8> {
        for i in 0..40 {
            let row = self.main_board.v[i];
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
        let idx = self.replay.replay_slices.len() as u32;
        let new_seed = accept_event(&self.seed, event, event_time, idx);
        let new_slice = GameReplaySlice {
            idx,
            event: event.clone(),
            new_seed,
            event_timestamp: event_time,
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

        if self.game_over {
            log::warn!("game over but you called put_next_cs");
            anyhow::bail!("game already over");
        }

        self.clear_line();
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
            self.game_over = true;
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
        self.score -= (soft_drops * 2) as i64;
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
        if self.game_over {
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
        if !new.game_over {
            new.put_ghost();
        }
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
                let value = (&self.main_board.v)[y][x];
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
                let old_value = (&self.main_board.v)[y][x];
                if old_value.eq(&CellValue::Ghost) {
                    self.main_board.v[y][x] = CellValue::Empty;
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

                if state1.game_over {
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
                if active_game.game_over {
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
