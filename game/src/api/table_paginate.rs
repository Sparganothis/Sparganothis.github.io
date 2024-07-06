use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TablePaginateDirection<T: Debug + Clone> {
    Forward(T),
    Back(T),
    InitialPage,
}
