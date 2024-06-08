enum RotDirection {
    Left,
    Right,
}
pub type Shape = Vec<Vec<bool>>;
fn rotate_shape(orig: Shape, rot: RotDirection) -> Shape {
    let mut new = vec![];

    new
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rot_i_left() {
        let result = rotate_shape(crate::tet::Tet::I.shape(), RotDirection::Left);
        let expected = vec![vec![true], vec![true], vec![true], vec![true]];
        assert_eq!(result, expected);
    }
}
