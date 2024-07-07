use pyo3::prelude::*;
use game::{random::{get_random_seed, GameSeed}, tet::{GameState, TetAction}, timestamp::get_timestamp_now_nano};

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
impl GameSeedPy {
    #[getter]
    fn ts(&self) ->  PyResult<i64> {
        Ok(self.ts)
    }
    
    #[getter]
    fn seed(&self) ->  PyResult<[u8;32]> {
        Ok(self.seed)
    }
}

#[pymethods]
impl GameStatePy {
    #[new]
    fn new(value: &GameSeedPy) -> Self {
        GameStatePy{inner: GameState::new(&value.seed, value.ts)}
    }
    
    #[getter]
    fn main_board(&self) -> PyResult<Vec<Vec<bool>>> {
        let mut brows = vec![];

        for row in self.inner.main_board.rows() {
            brows.push(row.iter().map(|x| match x {
                game::tet::CellValue::Piece(_) => true,
                game::tet::CellValue::Garbage => true,
                game::tet::CellValue::Empty => false,
                game::tet::CellValue::Ghost => false,
            }).collect());
        }
        Ok(brows)
    }

    #[getter]
    fn next_pcs(&self) -> PyResult<Vec<String>> {
        let mut v = vec![];
        
        for x in self.inner.next_pcs.iter().take(5) {
            v.push(x.name().to_string());
        }
        Ok(v)
    }
    // pub total_lines: i64,
    #[getter]
    fn total_lines(&self) ->  PyResult<i64> {
        Ok(self.inner.total_lines)
    }

    #[getter]
    fn hold(&self) ->  PyResult<Option<String>> {
        Ok(self.inner.hold_pcs.clone().map(|x| x.tet.name().to_string()))
    }

    #[getter]
    fn game_over(&self) ->  PyResult<bool> {
        Ok(self.inner.game_over)
    }

    #[getter]
    fn score(&self) ->  PyResult<i64> {
        Ok(self.inner.score)
    }

    #[getter]
    fn debug_current_pcs_info(&self) -> PyResult<String> {
        Ok(format!("current_pcs: {:?}", self.inner.current_pcs))
    }

    #[getter]
    fn get_current_pcs_rotation(&self) -> PyResult<(String, i64, (i8, i8))> {

        if let Some(c) = self.inner.current_pcs {
            let rot = match c.rs {
                game::rot::RotState::R0 => 0,
                game::rot::RotState::R1 => 1,
                game::rot::RotState::R2 =>2,
                game::rot::RotState::R3 => 3,
            };
            let name = c.tet.name().to_string();
            let pos = c.pos;
            Ok((name, rot, pos))
        } else {
            Ok(("".to_string(), -666, (-127, -127)))
        }
    }

    #[getter]
    pub fn next_actions_and_states(&self) -> PyResult<Vec<(String, GameStatePy)>> {
        let mut v = vec![];

        for action in TetAction::all() {
            if let Ok(result) = self.inner.try_action(action, 0) {
                v.push((action.name(), GameStatePy{inner:result}));
            }
        }
        Ok(v)
    }
}

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
