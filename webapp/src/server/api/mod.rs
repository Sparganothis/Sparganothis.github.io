pub mod server_info;
pub mod user;
use  serde::{Serialize,Deserialize};
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Count {
    pub value: i32,
}
