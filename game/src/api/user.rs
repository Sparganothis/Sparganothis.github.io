use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, PartialEq, PartialOrd, Eq, Ord, Hash,
)]
pub struct GuestInfo {
    pub user_id: uuid::Uuid,
    pub pageviews: usize,
    pub first_seen: OffsetDateTime,
    pub last_seen: OffsetDateTime,
}

impl Default for GuestInfo {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            pageviews: 0,
            first_seen: OffsetDateTime::now_utc(),
            last_seen: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct UserProfile {
    pub display_name: String,
}
