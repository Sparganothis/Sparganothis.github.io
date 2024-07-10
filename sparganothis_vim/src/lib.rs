use pyo3::{exceptions::PyValueError, prelude::*};
use game::{bot::random_choice_bot::get_all_move_chains, random::{get_random_seed, GameSeed}, tet::{segments_to_states, GameReplaySegment, GameState, TetAction}, timestamp::get_timestamp_now_nano};

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

        for row in self.inner.main_board.rows().into_iter().take(20) {
            brows.push(row.iter().map(|x| match x {
                game::tet::CellValue::Piece(_) => true,
                game::tet::CellValue::Garbage => true,
                game::tet::CellValue::Empty => false,
                game::tet::CellValue::Ghost => false,
            }).collect());
        }
        
        brows.reverse();
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
    fn is_t_spin(&self) ->  PyResult<bool> {
        Ok(self.inner.is_t_spin)
    } 

    #[getter]
    fn is_t_mini_spin(&self) ->  PyResult<bool> {
        Ok(self.inner.is_t_mini_spin)
    }
    
    #[getter]
    fn is_b2b(&self) ->  PyResult<bool> {
        Ok(self.inner.is_b2b)
    }
    #[getter]
    fn combo_counter(&self) ->  PyResult<i32> {
        Ok(self.inner.combo_counter)
    }
    #[getter]
    fn total_garbage_sent(&self) ->  PyResult<i64> {
        Ok(self.inner.total_garbage_sent)
    }
    #[getter]
    fn garbage_recv(&self) ->  PyResult<i64> {
        Ok(self.inner.garbage_recv)
    }
    #[getter]
    fn bumpi(&self) ->  PyResult<i32> {
        let mut x = self.inner.clone();
        if let Some(c) = x.current_pcs {
            let _ = x.main_board.delete_piece(&c);
            Ok(x.main_board.board_bumpi())
        } else {
            Ok(self.inner.main_board.board_bumpi())
        }
    }
    #[getter]
    fn holes(&self) ->  PyResult<i32> {
        let mut x = self.inner.clone();
        if let Some(c) = x.current_pcs {
            let _ = x.main_board.delete_piece(&c);
            Ok(x.main_board.board_holes())
        } else {
            Ok(self.inner.main_board.board_holes())
        }
    }
    #[getter]
    fn height(&self) ->  PyResult<i32> {
        let mut x = self.inner.clone();
        if let Some(c) = x.current_pcs {
            let _ = x.main_board.delete_piece(&c);
            Ok(x.main_board.get_height()+1)
        }
        else         {
            Ok(self.inner.main_board.get_height()+1)
        }
    }

    #[getter]
    fn matrix_txt(&self) ->  PyResult<String> {
        let mut matrix_rows = vec![];

        matrix_rows.push("\n-------------".to_string());
        for (i, row) in self.main_board()?.into_iter().enumerate() {
            let row_str: Vec<_> = row.iter().map(|x| if *x {"x"} else {" "}.to_string()).collect();
            let row_str = row_str.join("");
            let row_extra = match i {
                1 => format!("current_pcs = {:?}", self.current_pcs()?),
                2 => format!("game_over            = {:?}", self.game_over()?),
                3 => format!("hold                 = {:?}", self.hold()?),
                4 => format!("next_pcs             = {:?}", self.next_pcs()?),
                5 => format!("total_lines          = {:?}", self.total_lines()?),
                6 => format!("is_t_spin            = {}", self.is_t_spin()?),
                7 => format!("is_t_mini_spin       = {}", self.is_t_mini_spin()?),
                8 => format!("is_b2b               = {}", self.is_b2b()?),
                9 => format!("combo_counter        = {}", self.combo_counter()?),
                10 => format!("total_garbage_sent  = {}", self.total_garbage_sent()?),
                11 => format!("garbage_recv        = {}", self.garbage_recv()?),
                12 => format!("total_move_count    = {}", self.total_move_count()?),
                13 => format!("bumpi               = {}", self.bumpi()?),
                14 => format!("holes               = {}", self.holes()?),
                15 => format!("height               = {}", self.height()?),
                16 => format!("bot_moves_raw('wordpress') = {:?}", self.bot_moves_raw("wordpress".to_string())?),
                17 => format!("bot_moves_raw('random') = {:?}", self.bot_moves_raw("random".to_string())?),
                18 => format!("get_valid_move_chains().len() = {:?} / {:?}", self.get_valid_move_chains()?.len(), get_all_move_chains().len()),
                _ => "".to_string()
            };
            matrix_rows.push(format!(" | {row_str} | {row_extra}"));
        }
        matrix_rows.push("-------------\n".to_string());
        Ok(matrix_rows.join("\n"))
    }

    #[getter]
    fn html(&self) -> PyResult<String> {
        let x = self.matrix_txt()?;
        Ok(format!("<code><pre>{x}</pre></code>"))
    }

    #[getter]
    fn total_move_count(&self) -> PyResult<i32> {
        Ok(self.inner.total_moves)
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
    fn current_pcs_rotation(&self) -> PyResult<(String, i64, (i8, i8))> {

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
    fn current_pcs(&self) -> PyResult<(String, i64, (i8, i8))> {
        self.current_pcs_rotation()
    }

    #[getter]
    pub fn next_actions_and_states(&self) -> PyResult<Vec<(String, GameStatePy)>> {
        let mut v = vec![];

        for action in TetAction::all() {
            if let Ok(mut result) = self.inner.try_action(action, 0) {
                result.replay.replay_slices.clear();
                v.push((action.name(), GameStatePy{inner:result}));
            }
        }
        Ok(v)
    }

    pub fn bot_moves_raw(&self, bot_type: String,) -> PyResult<Vec<String>> {

        let b = game::bot::get_bot(&bot_type).map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        let moves = b.choose_move(&self.inner).map_err(|e| PyValueError::new_err(format!("bad bot move: {}", e)))?;
        Ok(moves.into_iter().map(|m| m.name()).collect())
    }

    pub fn generate_bot_episode(&self, bot_type: String, max_episode_len: usize) -> PyResult<Vec<(String, GameStatePy)>> {
        let mut v = vec![];

        let b = game::bot::get_bot(&bot_type).map_err(|e| PyValueError::new_err(format!("{}", e)))?;
        let mut state = self.inner.clone();
        let mut _i = 0;
        while _i < max_episode_len {
            state.replay.replay_slices.clear();
            if state.game_over {
                break
            }
            match b.choose_move(&state) {
                Ok(actions) => {
                    if actions.is_empty() {
                        break
                    }
                    for act in actions {

                        if state.apply_action_if_works(act, 0).is_err() {
                            break
                        }
                        state.replay.replay_slices.clear();
                        v.push((act.name(), GameStatePy{inner: state.clone()}));
                        _i += 1;
                        if _i >= max_episode_len {
                            break
                        }
                    }
                }
                Err(_) => break
            }
        }
        Ok(v)
    }

    #[staticmethod]
    fn get_all_move_chains() -> PyResult<Vec<Vec<String>>> {
        Ok(get_all_move_chains().into_iter().map(|x| x.into_iter().map(|y| y.name()).collect()).collect())
    }

    #[staticmethod]
    fn load_replay_from_bytes(data: Vec<u8>) -> PyResult<(GameStatePy, Vec<(String, GameStatePy)>)> {
        let segments: Vec<GameReplaySegment> = bincode::deserialize(&data).map_err(|e| PyValueError::new_err(format!("bad data: {}", e)))?;
        if segments.len() < 3 {
            return Err(PyValueError::new_err("not enough segments in savefile."))
        }
        let states = segments_to_states(&segments);
        if states.len() < 3 {
            return Err(PyValueError::new_err("not enough gamestates in savefile."))
        }
        let s1 = states.first().unwrap();
        let ps1 = GameStatePy{inner:s1.clone()};
        let mut v = vec![];

        for i in 0..(segments.len().min(states.len())) {
            let st = &segments[i];
            let mut gt = (&states[i]).clone();
            gt.replay.replay_slices.clear();
            match st {
                GameReplaySegment::Update(_x) => {
                    let ev = _x.event.action.name();
                    let pgt = GameStatePy{inner: gt.clone()};
                    v.push((ev, pgt));
                },
                _ => continue
            }
        }

        Ok((ps1, v))
    }

    fn get_valid_move_chains(&self) -> PyResult<Vec<(Vec<String>, GameStatePy)>> {
        let mut v = vec![];

        let state = self.inner.clone();
        for t in get_all_move_chains() {
            let mut s_current = state.clone();
            let mut s_ok = true;
            for action in t.clone() {
                if s_current.apply_action_if_works(action, 0).is_err() {
                    s_ok = false;
                    break
                }
            }
            if s_ok {
                let r: Vec<String> = t.into_iter().map(|x| x.name()).collect();
                s_current.replay.replay_slices.clear();
                v.push((r, GameStatePy{inner:s_current}));
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
