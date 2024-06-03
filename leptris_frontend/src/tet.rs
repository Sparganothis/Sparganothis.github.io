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
    pub fn debug_spawn_nextpcs(&mut self) {
        let col: i8 = 0;
        let mut row: i8 = R as i8 - 3;
        for piece in Tet::all() {
            let r = self.spawn_piece(piece, (row, col));
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

pub const SIDE_BOARD_WIDTH: usize = 4;
type BoardMatrixHold = BoardMatrix<3, SIDE_BOARD_WIDTH>;
type BoardMatrixNext = BoardMatrix<21, SIDE_BOARD_WIDTH>;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GameState {
    pub main_board: BoardMatrix,
    pub next_board: BoardMatrixNext,
    pub hold_board: BoardMatrixHold,
    pub last_action: TetAction,
}
impl GameState {
    pub fn empty() -> Self {
        Self {
            main_board: BoardMatrix::empty(),
            next_board: BoardMatrixNext::empty(),
            hold_board: BoardMatrixHold::empty(),
            last_action: TetAction::Nothing,
        }
    }

    pub fn try_action(&self, action: TetAction) -> anyhow::Result<Self> {
        let mut new = self.clone();
        new.last_action = action;
        Ok(new)
    }
}
