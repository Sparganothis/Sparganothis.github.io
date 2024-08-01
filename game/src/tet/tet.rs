
use rand::Rng;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use super::rot::{RotState, Shape};


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


pub static ALL_SHAPES: Lazy<std::collections::HashMap<(RotState, Tet), Shape>> =
    Lazy::new(|| {
        let mut h = std::collections::HashMap::<_, _>::new();
        for t in Tet::all() {
            for r in [RotState::R0, RotState::R1, RotState::R2, RotState::R3] {
                let key = (r, t);
                let val = t.make_shape(r);
                h.insert(key, val);
            }
        }
        h
    });



pub const SPAWN_POS: (i8, i8) = (18, 3);

impl Tet {
    #[inline(always)]
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

    #[inline(always)]
    pub fn shape(&self, rot_state: super::rot::RotState) -> Shape {
        ALL_SHAPES
            .get(&(rot_state, *self))
            .expect("rot shape combo not found; lazy not initialized.")
            .to_owned()
    }

    fn make_shape(&self, rot_state: super::rot::RotState) -> Shape {
        let mut sh = self.make_orig_shape();
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

    fn make_orig_shape(&self) -> Shape {
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
    #[inline(always)]
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