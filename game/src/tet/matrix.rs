
use serde::{Deserialize, Serialize};

use super::random::{get_determinist_val, GameSeed};

use super::{game_state::CurrentPcsInfo, rot::RotState, tet::Tet};
use serde_with::serde_as;



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CellValue {
    // 4 bit after, before 16-20bit b
    Piece(Tet),
    Garbage,
    Empty,
    Ghost,
}

impl CellValue {
    // This has to be a const fn
    #[inline(always)]
    const fn into_bits(self) -> u8 {
        match self {
            CellValue::Empty => 0,
            CellValue::Piece(Tet::I) => 1,
            CellValue::Piece(Tet::L) => 2,
            CellValue::Piece(Tet::J) => 3,
            CellValue::Piece(Tet::T) => 4,
            CellValue::Piece(Tet::S) => 5,
            CellValue::Piece(Tet::Z) => 6,
            CellValue::Piece(Tet::O) => 7,
            CellValue::Garbage => 8,
            CellValue::Ghost => 9,
        }
    }
    #[inline(always)]
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => CellValue::Empty,
            1 => CellValue::Piece(Tet::I),
            2 => CellValue::Piece(Tet::L),
            3 => CellValue::Piece(Tet::J),
            4 => CellValue::Piece(Tet::T),
            5 => CellValue::Piece(Tet::S),
            6 => CellValue::Piece(Tet::Z),
            7 => CellValue::Piece(Tet::O),
            8 => CellValue::Garbage,
            9 => CellValue::Ghost,
            _ => CellValue::Empty,
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMatrix<const R: usize = 40, const C: usize = 10> {
    // 400 * cellValue = 1600bit after / 8000 before -- 200byte after, 1k before
    // with no color -- 400bit = 80bytes
    #[serde_as(as = "[_; R]")]
    vv: [CellValueRow10; R],
}


pub const SIDE_BOARD_WIDTH: usize = 4;
pub type BoardMatrixHold = BoardMatrix<3, SIDE_BOARD_WIDTH>;
pub type BoardMatrixNext = BoardMatrix<16, SIDE_BOARD_WIDTH>;


#[bitfield_struct::bitfield(u8, order = Msb)]
#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellValuePairByte {
    #[bits(4)]
    val0: CellValue,
    #[bits(4)]
    val1: CellValue,
}
impl CellValuePairByte {
    #[inline(always)]
    fn empty() -> Self {
        Self::new()
            .with_val0(CellValue::Empty)
            .with_val1(CellValue::Empty)
    }
    #[inline(always)]
    fn get(&self, idx: i8) -> CellValue {
        match idx {
            0 => self.val0(),
            1 => self.val1(),
            _ => panic!("invalid index {idx} not in [0,1]"),
        }
    }
    #[inline(always)]
    fn set(&mut self, idx: i8, new: CellValue) {
        *self = match idx {
            0 => self.with_val0(new),
            1 => self.with_val1(new),
            _ => panic!("invalid index {idx} not in [0,1]"),
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellValueRow10 {
    #[serde_as(as = "[_; 5]")]
    v_r: [CellValuePairByte; 5],
}

impl CellValueRow10 {
    #[inline(always)]
    fn empty() -> Self {
        Self {
            v_r: [CellValuePairByte::empty(); 5],
        }
    }
    #[inline(always)]
    fn get(&self, idx: i8) -> CellValue {
        assert!(idx >= 0 && idx <= 9, "bad idx: {idx} expected: 0..=4");
        self.v_r[idx as usize / 2].get(idx % 2)
    }
    #[inline(always)]
    fn set(&mut self, idx: i8, new: CellValue) {
        assert!(idx >= 0 && idx <= 9, "bad idx: {idx} expected: 0..=4");
        self.v_r[idx as usize / 2].set(idx % 2, new);
    }
    #[inline(always)]
    fn to_cells(&self) -> [CellValue; 10] {
        [
            self.v_r[0].get(0),
            self.v_r[0].get(1),
            self.v_r[1].get(0),
            self.v_r[1].get(1),
            self.v_r[2].get(0),
            self.v_r[2].get(1),
            self.v_r[3].get(0),
            self.v_r[3].get(1),
            self.v_r[4].get(0),
            self.v_r[4].get(1),
        ]
    }
}

impl<const R: usize, const C: usize> BoardMatrix<R, C> {
    pub fn clear_line(&mut self, line: i8) {
        for i in line..39 {
            self.vv[i as usize] = self.vv[i as usize + 1];
        }
    }
    pub fn inject_single_garbage_line(&mut self, seed: GameSeed) {
        let v: u8 = get_determinist_val::<u8>(&seed) % C as u8;

        // move all things up
        for i in (0..(R - 2)).rev() {
            self.vv[i as usize + 1] = self.vv[i as usize];
        }

        for x in 0..(C as i8) {
            if x != (v as i8) {
                self.set_cell(0, x, CellValue::Garbage)
            } else {
                self.set_cell(0, x, CellValue::Empty)
            }
        }
    }
    #[inline(always)]
    pub fn get_cell(&self, y: i8, x: i8) -> Option<CellValue> {
        if x < 0 || y < 0 || x >= (C as i8) || y >= (R as i8) {
            None
        } else {
            Some(self.vv[y as usize].get(x as i8))
        }
    }

    #[inline(always)]
    pub fn set_cell(&mut self, y: i8, x: i8, v: CellValue) {
        self.vv[y as usize].set(x as i8, v);
    }

    pub fn rows(&self) -> Vec<Vec<CellValue>> {
        self.vv
            .iter()
            .map(|r| r.to_cells().iter().take(C).cloned().collect())
            .collect()
    }
    #[inline(always)]
    pub fn empty() -> Self {
        Self {
            vv: [CellValueRow10::empty(); R],
        }
    }

    #[inline(always)]
    pub fn get_num_rows(&self) -> usize {
        R
    }
    #[inline(always)]
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
    pub fn spawn_nextpcs(&mut self, next_pcs: &Vec<Tet>) {
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
                CellValue::Piece(_) => return x as i32,
                CellValue::Garbage => return x as i32,
                CellValue::Empty => continue,
                CellValue::Ghost => continue,
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
                    CellValue::Empty |CellValue::Ghost => {
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