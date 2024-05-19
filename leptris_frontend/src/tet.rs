use std::ops::RangeBounds;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tet {
    I, L, J, T, S, Z, O
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
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CellValue {
    Piece(Tet),
    Garbage,
    Empty
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Board<const R: usize = 40, const C: usize = 10>{
    v: [[CellValue; C]; R]
}

impl<const R: usize, const C: usize> Board<R, C> {
    pub fn empty() -> Self {
        Self {
            v: [[CellValue::Empty; C]; R]
        }
    }
    pub fn rows(&self) -> Vec<Vec<CellValue>> {
        self.v.iter().map(|r| r.into_iter().cloned().collect()).collect()
    }
}