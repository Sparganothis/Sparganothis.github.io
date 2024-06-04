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

    pub fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            &Self::I => vec![vec![true, true, true, true]],
            &Self::L => vec![vec![true, true, true], vec![false, false, true]],
            &Self::J => vec![vec![true, true, true], vec![true, false, false]],
            &Self::T => vec![vec![true, true, true], vec![false, true, false]],
            &Self::S => vec![vec![true, true, false], vec![false, true, true]],
            &Self::Z => vec![vec![false, true, true], vec![true, true, false]],
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

    pub fn all_shuffled() -> Vec<Self> {
        let mut v = Self::all();
        use rand::thread_rng;
        let mut rng = thread_rng();
        use rand::prelude::SliceRandom;
        v.shuffle(&mut rng);
        v
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
    pub fn spawn_piece(&mut self, piece: Tet, (y, x): (i8, i8)) -> anyhow::Result<()> {
        if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
            anyhow::bail!(
                "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})"
            );
        }
        let (x, y) = (x as usize, y as usize);
        let shape = piece.shape();
        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                let (cx, cy) = (x + i, y + j);
                if cx >= C || cy >= R {
                    anyhow::bail!("computed position out of game bounds (got (x={cx} y={cy}), max (x={C} y={R})")
                }
                if *cell {
                    match self.v[cy][cx] {
                        CellValue::Empty => {
                        }
                        CellValue::Garbage | CellValue::Piece(_) => {
                            anyhow::bail!("cell position already taken");
                        }
                    }
                }
            }
        }

        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                let (cx, cy) = (x + i, y + j);
                if cx >= C || cy >= R {
                    anyhow::bail!("computed position out of game bounds (got (x={cx} y={cy}), max (x={C} y={R})")
                }
                if *cell {
                    match self.v[cy][cx] {
                        CellValue::Empty => {
                            self.v[cy][cx] = CellValue::Piece(piece);
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

    pub fn delete_piece(&mut self, piece: Tet, (y, x): (i8, i8)) -> anyhow::Result<()> {
        if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
            anyhow::bail!(
                "given position out of game bounds (got (x={x} y={y}), max (x={C} y={R})"
            );
        }
        let (x, y) = (x as usize, y as usize);
        let shape = piece.shape();
        for (j, row) in shape.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                let (cx, cy) = (x + i, y + j);
                if cx >= C || cy >= R {
                    anyhow::bail!("computed position out of game bounds (got (x={cx} y={cy}), max (x={C} y={R})")
                }
                if *cell {
                    self.v[cy][cx] = CellValue::Empty;
                }
            }
        }
        Ok(())
    }
    pub fn spawn_nextpcs(&mut self, next_pcs: &VecDeque<Tet>) {
        let col: i8 = 0;
        let mut row: i8 = R as i8 - 2;
        for (i, piece) in next_pcs.iter().enumerate() {
            if i >= 5 {
                break;
            }
            let r = self.spawn_piece(*piece, (row, col));
            row -= 3;
            if r.is_err() {
                log::info!("{r:?}");
            }
        }
    }
    pub fn rows(&self) -> Vec<Vec<CellValue>> {
        self.v
            .iter()
            .map(|r| r.into_iter().cloned().collect())
            .collect()
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
        let choices = vec![
            Self::HardDrop,
            Self::SoftDrop,
            Self::MoveLeft,
            Self::MoveRight,
            Self::Hold,
            Self::RotateLeft,
            Self::RotateRight,
            // Nothing is not action
        ];
        let mut rng = thread_rng();
        *choices.choose(&mut rng).unwrap()
    }
}

pub const SIDE_BOARD_WIDTH: usize = 4;
type BoardMatrixHold = BoardMatrix<3, SIDE_BOARD_WIDTH>;
type BoardMatrixNext = BoardMatrix<16, SIDE_BOARD_WIDTH>;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameState {
    pub main_board: BoardMatrix,
    // pub next_board: BoardMatrixNext,
    // pub hold_board: BoardMatrixHold,
    pub last_action: TetAction,
    pub next_pcs: VecDeque<Tet>,
    pub current_pcs: Option<CurrentPcsInfo>,
    pub current_id: u32,

    pub hold_pcps: Option<HoldPcsInfo>,
    pub game_over: bool,
}

const SPAWN_POS: (i8, i8) = (19, 4);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HoldPcsInfo {
    can_use: bool,
    tet: Tet,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CurrentPcsInfo {
    pos: (i8, i8),
    tet: Tet,
    id: u32,
}

impl GameState {
    pub fn get_debug_info(&self) -> String {
        format!(
            "last_acction: {:?}  \n\n next_pcs: {:?}  current_pcs: {:?} \n\n, hold_psc: {:?}",
            self.last_action, self.next_pcs, self.current_pcs, self.hold_pcps
        )
    }
    fn put_next_piece(&mut self) {
        if self.current_pcs.is_some() {
            log::warn!("cannont put next pcs because we already have one");
            return;
        }

        if self.game_over {
            log::warn!("game over but you called put_next_cs");
            return;
        }

        if self.next_pcs.len() < 7 {
            let new_pcs2 = Tet::all_shuffled();
            for n in new_pcs2 {
                self.next_pcs.push_back(n);
            }
        }
        let next_tet = self.next_pcs.pop_front().unwrap();

        self.current_pcs = Some(CurrentPcsInfo {
            pos: SPAWN_POS,
            tet: next_tet,
            id: self.current_id,
        });
        self.current_id += 1;

        if let Err(_) = self.main_board.spawn_piece(next_tet, SPAWN_POS) {
            self.game_over = true;
        } else {
            if let Some(ref mut h) = self.hold_pcps {
                h.can_use = true;
            }
        }
    }
    pub fn empty() -> Self {
        let mut next_pcs = Tet::all();
        use rand::thread_rng;
        let mut rng = thread_rng();
        use rand::prelude::SliceRandom;

        next_pcs.shuffle(&mut rng);

        let mut new_state = Self {
            main_board: BoardMatrix::empty(),
            // next_board: BoardMatrixNext::empty(),
            // hold_board: BoardMatrixHold::empty(),
            last_action: TetAction::Nothing,
            next_pcs: next_pcs.into(),
            current_pcs: None,
            game_over: false,
            hold_pcps: None,
            current_id: 0,
        };
        new_state.put_next_piece();
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
            log::info!("try  to hold fuck");
            if let Err(e) = b.spawn_piece(tet, (1, 0)) {
                log::warn!("hold board cannot spawn piece WTF: {:?}", e);
            }
        }
        b
    }

    pub fn try_hold(&mut self) -> anyhow::Result<()> {
        if self.current_pcs.is_none() {
            anyhow::bail!("no cucrrent pcs for hold");
        }
        let current_pcs = self.current_pcs.clone().unwrap();

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

        if let Err(e) = self
            .main_board
            .delete_piece(current_pcs.tet, current_pcs.pos)
        {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }
        self.current_pcs = None;

        if let Some(ref old_hold) = old_hold {
            self.next_pcs.push_front(old_hold.tet);
        }
        self.put_next_piece();
        self.hold_pcps = Some(HoldPcsInfo {
            tet: current_pcs.tet,
            can_use: false,
        });

        Ok(())
    }

    pub fn try_harddrop(&mut self) -> anyhow::Result<()> {
        let current_pcs = self.current_pcs.clone().unwrap();

        let mut r = self.try_softdrop();
        while r.is_ok() && current_pcs.id == self.current_pcs.clone().unwrap().id {
            r = self.try_softdrop();
        }
        Ok(())
    }

    pub fn try_softdrop(&mut self) -> anyhow::Result<()> {
        if self.current_pcs.is_none() {
            anyhow::bail!("no cucrrent pcs for hold");
        }
        
        let current_pcs = self.current_pcs.clone().unwrap();

        if let Err(e) = self
            .main_board
            .delete_piece(current_pcs.tet, current_pcs.pos)
        {
            log::warn!("ccannot delete picei from main board plz: {:?}", e)
        }

        let mut new_current_pcs = current_pcs.clone();
        new_current_pcs.pos.0 -= 1;

        if self.main_board.spawn_piece(new_current_pcs.tet, new_current_pcs.pos).is_ok() {
            self.current_pcs = Some(new_current_pcs);
        } else {
            self.main_board.spawn_piece(current_pcs.tet, current_pcs.pos).unwrap();
            self.current_pcs = None;
            self.put_next_piece();
        }
        Ok(())
    }

    pub fn try_action(&self, action: TetAction) -> anyhow::Result<Self> {
        if self.game_over {
            log::warn!("gamem over cannot try_action");
            anyhow::bail!("game over");
        }
        let mut new = self.clone();
        new.last_action = action;

        match action {
            TetAction::HardDrop => {
                if let Err(err) = new.try_harddrop() {
                    log::warn!("hard drop ailed: {:?}", err);
                }
            }
            TetAction::SoftDrop => {
                if let Err(err) = new.try_softdrop() {
                    log::warn!("hold failed: {:?}", err);
                }
            }         TetAction::MoveLeft => {}
            TetAction::MoveRight => {}
            TetAction::Hold => {
                if let Err(err) = new.try_hold() {
                    log::warn!("hold failed: {:?}", err);
                }
            }
            TetAction::RotateLeft => {}
            TetAction::RotateRight => {}
            TetAction::Nothing => {}
        }
        Ok(new)
    }

    pub fn apply_action_if_works(action: TetAction, target: &mut Self) -> anyhow::Result<()> {
        let r = target.try_action(action);
        if let Ok(new_state) = r {
            *target = new_state;
            Ok(())
        } else {
            let e = r.unwrap_err();
            log::warn!("user action {:?} failed: {:?}", action, e);
            Err(e)
        }
    }
}
