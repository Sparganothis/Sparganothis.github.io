#![deny(unused_crate_dependencies)]
// for "wasm-pack test --node"
use getrandom as _;
use wasm_bindgen_test as _;

pub mod api;
pub mod bot;
pub mod random;
pub mod rot;
pub mod tet;
pub mod timestamp;

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    pub fn test_2_plus_2() {
        assert_eq!(2 + 2, 4)
    }
}
