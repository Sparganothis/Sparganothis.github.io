use pyo3::prelude::*;
use game::{random::{get_random_seed, GameSeed}, tet::GameState, timestamp::get_timestamp_now_nano};

#[pyclass]
struct GameStatePy {
    pub inner: GameState,
}

// 


#[pyclass]
struct GameSeedPy{
    pub seed: GameSeed,
    pub ts: i64,
}

#[pymethods]
impl GameStatePy {
    #[new]
    fn new(value: &GameSeedPy) -> Self {
        GameStatePy{inner: GameState::new(&value.seed, value.ts)}
    }

    #[getter]
    fn score(&self) -> PyResult<i64> {
        Ok(self.score)
    }
    
    #[getter]
    fn main_board(&self) -> PyResult<Vec<bool>> {
        let mut v = vec![];

        Ok(v)
    }
}

// pub main_board: BoardMatrix,
// // pub next_board: BoardMatrixNext,
// // pub hold_board: BoardMatrixHold,
// pub last_action: TetAction,
// pub next_pcs: VecDeque<Tet>,
// pub current_pcs: Option<CurrentPcsInfo>,
// pub current_id: u32,

// pub hold_pcs: Option<HoldPcsInfo>,
// pub game_over: bool,

// pub replay: GameReplay,
// pub seed: GameSeed,
// pub init_seed: GameSeed,
// pub start_time: i64,
// pub total_lines: i64,

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn generate_seed(ts: i64, seed: [u8;32]) -> PyResult<GameSeedPy> {
    Ok(GameSeedPy{
        ts, seed})
}

#[pyfunction]
fn generate_random_seed() -> PyResult<GameSeedPy> {
    Ok(GameSeedPy{ts:get_timestamp_now_nano(), seed:get_random_seed()})
}

/// A Python module implemented in Rust.
#[pymodule]
fn sparganothis_vim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(generate_seed, m)?)?;
    m.add_function(wrap_pyfunction!(generate_random_seed, m)?)?;
    m.add_class::<GameStatePy>()?;
    m.add_class::<GameSeedPy>()?;
    Ok(())
}
