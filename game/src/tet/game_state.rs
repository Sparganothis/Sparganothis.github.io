use std::time::Duration;

use super::random::{accept_event, shuffle_tets, GameSeed};
use crate::timestamp::get_timestamp_now_nano;

use super::{
    matrix::{BoardMatrix, BoardMatrixHold, BoardMatrixNext, CellValue},
    rot::{RotDirection, RotState},
    tet::{Tet, TetAction},
};
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    pub current_pcs: Option<CurrentPcsInfo>, // 29 bit
    pub current_id: u16,                     // 13 bit

    pub hold_pcs: Option<HoldPcsInfo>, // 4 bit
    // pub game_over: bool,
    pub game_over_reason: Option<GameOverReason>, // 3 bit

    pub seed: GameSeed,      // 32 bytes = 256bit
    pub init_seed: GameSeed, // 256bit
    pub start_time: i64,     // n--ai acsf
    pub total_lines: u16,
    pub total_garbage_sent: u16, // 15 bit
    pub garbage_recv: u16,       // 15 bit
    pub garbage_applied: u16,
    pub total_moves: u16, // 16 bit

    pub last_segment: GameReplaySegment, // OK
    pub last_segment_idx: u16,

    // pub next_pcs: VecDeque<Tet>,             // 42 bit
    pub next_pcs_bags: [Tet; 14],
    pub next_pcs_idx: u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayInit {
    pub init_seed: GameSeed,
    pub start_time: i64,
}

impl GameReplayInit {
    pub fn empty(seed: &GameSeed, start_time: i64) -> Self {
        Self {
            init_seed: *seed,
            start_time,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameReplaySegment {
    Init(GameReplayInit),
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplaySlice {
    pub event_timestamp: i64,
    pub new_seed: GameSeed,
    pub new_garbage_recv: u16,
    pub new_garbage_applied: u16,
    pub idx: u16,
    pub event: GameReplayEvent,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameReplayEvent {
    pub action: TetAction,
    // pub game_over: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
        let (bag1, seed1) = shuffle_tets(&seed, start_time);
        let (bag2, seed2) = shuffle_tets(&seed1, start_time);
        let mut next_pcs_bags = [Tet::I; 14];
        for i in 0..7 {
            next_pcs_bags[i] = bag1[i];
            next_pcs_bags[i + 7] = bag2[i];
        }
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
            current_pcs: None,
            game_over_reason: None,
            hold_pcs: None,
            current_id: 0,
            seed: seed2,
            init_seed: *seed,
            last_segment: GameReplaySegment::Init(GameReplayInit::empty(
                seed, start_time,
            )),
            last_segment_idx: 0,
            start_time,
            total_lines: 0,
            total_garbage_sent: 0,
            garbage_recv: 0,
            total_moves: 0,
            garbage_applied: 0,
            next_pcs_idx: 0,
            next_pcs_bags,
        };
        let _ = new_state.put_next_piece(start_time, None);
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
            self.main_board.clear_line(line);

            lines += 1;
        }
        self.add_score_for_clear_line(lines);
        self.add_garbage_for_clear_line(lines);
        self.total_lines += lines;
    }

    fn add_garbage_for_clear_line(&mut self, lines: u16) {
        self.total_garbage_sent += match self.combo_counter {
            1 | 2 => 1,
            3 | 4 => 2,
            5 | 6 => 3,
            _i if _i >= 7 => 4,
            _ => 0,
        };

        self.total_garbage_sent += match lines {
            4 => 4,
            3 => 2,
            2 => 1,
            _ => 0,
        };

        if self.is_gameboard_empty() {
            self.total_garbage_sent += match lines {
                0 => 0,
                _ => 10,
            };
        }

        if self.is_t_spin {
            self.total_garbage_sent += match lines {
                1 => 2,
                2 => 4,
                3 => 6,
                _ => 0,
            };
        }
        if self.is_t_mini_spin {
            self.total_garbage_sent += match lines {
                1 => 1,
                2 => 3,
                3 => 5,
                _ => 0,
            };
        }
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
        let idx = self.last_segment_idx;
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
        self.last_segment = GameReplaySegment::Update(new_slice);
        self.last_segment_idx += 1;
    }

    fn refill_nextpcs(&mut self, event_time: i64) {
        log::info!("XXXX refill next pcs: {}", self.next_pcs_idx);
        if self.next_pcs_idx >= 7 {
            for i in 0..7 {
                self.next_pcs_bags[i] = self.next_pcs_bags[i + 7];
            }
            self.next_pcs_idx -= 7;
            // log::info!("next refill");
            let (new_pcs2, new_seed) = shuffle_tets(&self.seed, event_time);
            for (i, n) in new_pcs2.iter().enumerate() {
                self.next_pcs_bags[i + 7] = *n;
            }
            self.seed = new_seed;
        }
    }
    fn pop_next_pcs(&mut self, event_time: i64) -> Tet {
        self.refill_nextpcs(event_time);
        let v = self.next_pcs_bags[self.next_pcs_idx as usize];
        log::info!("XXXX pop next pcs += 1");
        self.next_pcs_idx += 1;
        v
    }
    fn put_next_piece(
        &mut self,
        _event_time: i64,
        maybe_next_pcs: Option<Tet>,
    ) -> anyhow::Result<()> {
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
        log::info!("XXXXX  caller next pcs: {:?}", maybe_next_pcs);
        let next_tet = match maybe_next_pcs {
            None => self.pop_next_pcs(_event_time),
            Some(x) => x,
        };

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
            self.last_segment = GameReplaySegment::GameOver(GameOverReason::Knockout);
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
        if let GameReplaySegment::Update(prev_slice) = &self.last_segment {
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
        if let GameReplaySegment::Update(self_slice) = &self.last_segment {
            if slice != self_slice {
                log::warn!(
                    "no  match in last slicec:  recieved == {:?},  rebuildt locally == ={:?}",
                    slice,
                    self_slice
                )
            }
        }
        Ok(())
    }
    pub fn get_next_board(&self) -> BoardMatrixNext {
        let mut b = BoardMatrixNext::empty();
        let vnext = self.get_next_pcs();
        b.spawn_nextpcs(&vnext);
        b
    }

    pub fn get_next_pcs(&self) -> Vec<Tet> {
        let mut v = Vec::<Tet>::new();
        for i in 0..5 {
            v.push(self.next_pcs_bags[self.next_pcs_idx as usize + i]);
        }
        v
    }

    pub fn set_next_pcs(&mut self, new: Vec<Tet>) {
        self.next_pcs_idx = 0;
        for i in 0..(new.len().min(14)) {
            self.next_pcs_bags[i] = new[i];
        }
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

        let maybe_old_hold = if let Some(ref old_hold) = old_hold {
            Some(old_hold.tet)
        } else {
            None
        };
        self.put_next_piece(event_time, maybe_old_hold)?;
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
            self.put_next_piece(event_time, None)?;
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
