use serde::{Deserialize, Serialize};

use super::tet::Tet;

pub fn srs_offsets(before: RotState, after: RotState, tet: Tet) -> Vec<(i8, i8)> {
    match tet {
        Tet::I => match (before, after) {
            (RotState::R0, RotState::R1) => {
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
            }
            (RotState::R1, RotState::R0) => {
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
            }
            (RotState::R1, RotState::R2) => {
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
            }
            (RotState::R2, RotState::R1) => {
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
            }
            (RotState::R2, RotState::R3) => {
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
            }
            (RotState::R3, RotState::R2) => {
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
            }
            (RotState::R3, RotState::R0) => {
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
            }
            (RotState::R0, RotState::R3) => {
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
            }
            _ => panic!("180 or 0 rot is bad"),
        },
        _ => match (before, after) {
            (RotState::R0, RotState::R1) => {
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            }
            (RotState::R1, RotState::R0) => {
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
            }
            (RotState::R1, RotState::R2) => {
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
            }
            (RotState::R2, RotState::R1) => {
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            }
            (RotState::R2, RotState::R3) => {
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            }
            (RotState::R3, RotState::R2) => {
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            }
            (RotState::R3, RotState::R0) => {
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            }
            (RotState::R0, RotState::R3) => {
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            }
            _ => panic!("180 or 0 rot is bad"),
        },
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotState {
    R0,
    R1,
    R2,
    R3,
}

impl RotState {
    pub fn rotate(&self, rot: RotDirection) -> Self {
        match (self, rot) {
            (&Self::R0, RotDirection::Left) => Self::R3,
            (&Self::R1, RotDirection::Left) => Self::R0,
            (&Self::R2, RotDirection::Left) => Self::R1,
            (&Self::R3, RotDirection::Left) => Self::R2,

            (&Self::R0, RotDirection::Right) => Self::R1,
            (&Self::R1, RotDirection::Right) => Self::R2,
            (&Self::R2, RotDirection::Right) => Self::R3,
            (&Self::R3, RotDirection::Right) => Self::R0,
        }
    }
}

pub type Shape = Vec<Vec<bool>>;
#[allow(non_snake_case)]
pub fn rotate_shape(shape: Shape, rot: RotDirection) -> Shape {
    let mut new_shape = vec![];

    #[allow(non_snake_case)]
    let R = shape.len();
    #[allow(non_snake_case)]
    let C = shape[0].len();
    match rot {
        RotDirection::Right => {
            for j in (0..C).rev() {
                let mut new_row: Vec<bool> = vec![];
                for i in 0..R {
                    new_row.push(shape[i][j]);
                }
                new_shape.push(new_row);
            }
        }
        RotDirection::Left => {
            for j in 0..C {
                let mut new_row: Vec<bool> = vec![];
                for i in (0..R).rev() {
                    new_row.push(shape[i][j]);
                }
                new_shape.push(new_row);
            }
        }
    }

    new_shape
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn rot_i_left() {
//         let result = rotate_shape(super::super::tet::Tet::I.orig_shape(), RotDirection::Left);
//         let expected = vec![vec![true], vec![true], vec![true], vec![true]];
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn rot_l_right() {
//         let result = rotate_shape(super::super::tet::Tet::L.orig_shape(), RotDirection::Right);
//         let expected = vec![vec![true, true], vec![true, false], vec![true, false]];
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn rot_l_left() {
//         let result = rotate_shape(super::super::tet::Tet::L.orig_shape(), RotDirection::Left);
//         let expected = vec![vec![false, true], vec![false, true], vec![true, true]];
//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn rot_j_right() {
//         let result = rotate_shape(super::super::tet::Tet::J.orig_shape(), RotDirection::Right);
//         let expected = vec![vec![true, false], vec![true, false], vec![true, true]];
//         assert_eq!(result, expected);
//     }

//     // #[test]
//     // fn rot_j_left() {
//     //     let result = rotate_shape(super::super::tet::Tet::J.orig_shape(), RotDirection::Right);
//     //     let expected = vec![vec![true, true], vec![false, true], vec![false, true]];
//     //     assert_eq!(result, expected);
//     // }
// }
