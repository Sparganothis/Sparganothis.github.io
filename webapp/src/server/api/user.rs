use leptos::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
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

#[server]
pub async fn who_am_i() -> Result<GuestInfo, ServerFnError> {
    use crate::server::backend::session::extract_guest_data;
    use crate::server::database::tables::get_or_create_user_profile;

    let guest_id = extract_guest_data().await?;
    get_or_create_user_profile(&guest_id.user_id).map_err(ServerFnError::new)?;

    Ok(guest_id)
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct UserProfile {
    pub display_name: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            display_name: random_word(),
        }
    }
}

#[cfg(feature = "ssr")]
fn random_word() -> String {
    random_word::gen(random_word::Lang::De).to_string()
}
#[cfg(not(feature = "ssr"))]
fn random_word() -> String {
    "".to_string()
}

#[server]
pub async fn get_profile(user_id: uuid::Uuid) -> Result<UserProfile, ServerFnError> {
    use crate::server::database::tables::get_user_profile;
    get_user_profile(&user_id).map_err(ServerFnError::new)
}
