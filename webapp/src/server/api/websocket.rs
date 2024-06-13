use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SocketType {
    Specctate(uuid::Uuid),
    Game1V1,
}
