use anyhow::Context;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::rot::{RotDirection, RotState, Shape};

use super::random::*;

use std::collections::VecDeque;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CellValue {
    Piece(Tet),
    Garbage,
    Empty,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BoardMatrix<const R: usize = 40, const C: usize = 10> {
    v: [[CellValue; C]; R],
}

impl<const R: usize, const C: usize> BoardMatrix<R, C> {
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
                        CellValue::Empty => {}
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
                        CellValue::Empty => {
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
    pub fn random() -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        if thread_rng().gen_bool(0.5) {
            Self::SoftDrop
        } else {
            let choices = [
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameState {
    pub score: i64,
    pub main_board: BoardMatrix,
    // pub next_board: BoardMatrixNext,
    // pub hold_board: BoardMatrixHold,
    pub last_action: TetAction,
    pub next_pcs: VecDeque<Tet>,
    pub current_pcs: Option<CurrentPcsInfo>,
    pub current_id: u32,

    pub hold_pcps: Option<HoldPcsInfo>,
    pub game_over: bool,

    pub replay: GameReplay,
    pub seed: GameSeed,
    pub init_seed: GameSeed,
    pub start_time: i64,
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
pub enum GameReplaySegment {
    Init(GameReplay),
    Update(GameReplaySlice),
}

impl GameReplaySegment {
    pub fn is_game_over(&self) -> bool {
        match self {
            Self::Update(slice) => slice.event.game_over,
            _ => false,
        }
    }
}

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
    pub game_over: bool,
}

const SPAWN_POS: (i8, i8) = (18, 3);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HoldPcsInfo {
    can_use: bool,
    tet: Tet,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CurrentPcsInfo {
    pos: (i8, i8),
    tet: Tet,
    rs: RotState,
    id: u32,
}

impl GameState {
    pub fn get_debug_info(&self) -> String {
        format!(
            "last_acction: {:?}  \n\n next_pcs: {:?}  current_pcs: {:?} \n\n, hold_psc: {:?}",
            self.last_action, self.next_pcs, self.current_pcs, self.hold_pcps
        )
    }

    fn clear_line(&mut self) {
        let mut score = 0;
        let mut lines = 0;
        while let Some(line) = self.can_clear_line() {
            for i in line..39 {
                for j in 0..10 {
                    self.main_board.v[i as usize][j] = self.main_board.v[i as usize + 1][j];
                }
            }
            lines += 1;
        }
        score += match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 0,
        };

        self.score += score;
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
            log::info!("next refill");
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
        } else if let Some(ref mut h) = self.hold_pcps {
            h.can_use = true;
        }
        Ok(())
    }

    pub fn accept_replay_slice(&mut self, slice: &GameReplaySlice) -> anyhow::Result<()> {
        log::info!("over={} acccept replay slice: {:?}", self.game_over, slice);
        *self = self.try_action(slice.event.action, slice.event_timestamp)?;
        let self_slicce = self.replay.replay_slices.last().unwrap();
        if !slice.eq(self_slicce) {
            log::warn!(
                "no  mat ch in last slicec:  recieved == {:?},  rebuildt locally == ={:?}",
                slice,
                self_slicce
            )
        }
        Ok(())
    }

    pub fn new(seed: &GameSeed, start_time: i64) -> Self {
        let mut new_state = Self {
            score: 0,
            main_board: BoardMatrix::empty(),
            // next_board: BoardMatrixNext::empty(),
            // hold_board: BoardMatrixHold::empty(),
            last_action: TetAction::Nothing,
            next_pcs: VecDeque::new(),
            current_pcs: None,
            game_over: false,
            hold_pcps: None,
            current_id: 0,
            seed: *seed,
            init_seed: *seed,
            replay: GameReplay::empty(seed, start_time),
            start_time,
        };
        new_state.refill_nextpcs(start_time);
        let _ = new_state.put_next_piece(start_time);
        new_state
    }

    pub fn get_next_board(&self) -> BoardMatrixNext {
        let mut b = BoardMatrixNext::empty();
        b.spawn_nextpcs(&self.next_pcs);
        b
    }

    pub fn get_hold_board(&self) -> BoardMatrixHold {
        let mut b = BoardMatrixHold::empty();
        if let Some(HoldPcsInfo { can_use: _, tet }) = self.hold_pcps {
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

        let old_hold = self.hold_pcps.clone();
        if let Some(ref old_hold) = old_hold {
            if !old_hold.can_use {
                anyhow::bail!("can_use=false for hold");
            }
        }

        self.hold_pcps = Some(HoldPcsInfo {
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
        self.hold_pcps = Some(HoldPcsInfo {
            tet: current_pcs.tet,
            can_use: false,
        });

        Ok(())
    }

    fn try_harddrop(&mut self, event_time: i64) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        let mut r = self.try_softdrop(event_time);
        while r.is_ok() && current_pcs.id == self.current_pcs.unwrap().id {
            r = self.try_softdrop(event_time);
        }
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
            self.current_pcs = Some(new_current_pcs);
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

    fn try_rotateleft(&mut self) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;
        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let mut new_current_pcs: CurrentPcsInfo = current_pcs;
        if (new_current_pcs.tet.name()=="I"){
            if new_current_pcs.pos.1==-2 {
                new_current_pcs.pos.1+=1;
            }
            else {
                if new_current_pcs.pos.1==7 {
                new_current_pcs.pos.1-=1;
            }
            }
        }
        if new_current_pcs.tet.name()!="O"{
            if new_current_pcs.pos.1==-1 {
                new_current_pcs.pos.1+=1;
            }
            else {
                if new_current_pcs.pos.1==8 {
                new_current_pcs.pos.1-=1;
            }
            }
        }
        new_current_pcs.rs = new_current_pcs.rs.rotate(RotDirection::Left);

        self.main_board.spawn_piece(&new_current_pcs)?;
        self.current_pcs = Some(new_current_pcs);
        Ok(())
    }

    fn try_rotateright(&mut self) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.context("no current pcs")?;

        if let Err(e) = self.main_board.delete_piece(&current_pcs) {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let mut new_current_pcs = current_pcs;
        if (new_current_pcs.tet.name()=="I"){
            if new_current_pcs.pos.1==-2 {
                new_current_pcs.pos.1+=1;
            }
            else {
                if new_current_pcs.pos.1==7 {
                new_current_pcs.pos.1-=1;
            }
            }
        }
        if (new_current_pcs.tet.name()!="O"){
            if new_current_pcs.pos.1==-1 {
                new_current_pcs.pos.1+=1;
            }
            else {
                if new_current_pcs.pos.1==8 {
                new_current_pcs.pos.1-=1;
            }
            }
        }
        new_current_pcs.rs = new_current_pcs.rs.rotate(RotDirection::Right);

        self.main_board.spawn_piece(&new_current_pcs)?;
        self.current_pcs = Some(new_current_pcs);
        Ok(())
    }

    fn try_action(&self, action: TetAction, event_time: i64) -> anyhow::Result<Self> {
        if self.game_over {
            log::warn!("gamem over cannot try_action");
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
                new.try_rotateleft()?;
            }
            TetAction::RotateRight => {
                new.try_rotateright()?;
            }
            TetAction::Nothing => {}
        }
        let ev = GameReplayEvent {
            action,
            game_over: self.game_over,
        };
        new.put_replay_event(&ev, event_time);
        Ok(new)
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
            log::warn!("user action {:?} failed: {:?}", action, e);
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
            184, 68, 55, 204, 137, 83, 38, 119, 235, 217, 151, 252, 189, 211, 31, 240, 177, 134,
            34, 165, 4, 237, 12, 233, 188, 242, 29, 154, 187, 93, 148, 100,
        ];
        assert_eq!(expected_seed, state.seed);

        state.apply_action_if_works(TetAction::HardDrop, 1).unwrap();

        let expected_seed = [
            63, 8, 223, 96, 71, 65, 34, 121, 209, 0, 184, 121, 139, 200, 63, 17, 128, 97, 128, 199,
            15, 96, 110, 237, 142, 156, 165, 26, 138, 216, 176, 245,
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
