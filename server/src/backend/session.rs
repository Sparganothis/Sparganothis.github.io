use async_trait::async_trait;
use axum::extract::FromRequestParts;
use game::api::user::GuestInfo;
use http::{request::Parts, StatusCode};
use time::OffsetDateTime;
use tower_sessions::{cookie::time::Duration, Session, SessionManagerLayer};
use tower_sessions_sled_store::SledStore;

pub struct Guest {
    _session: Session,
    pub guest_data: GuestInfo,
}

impl Guest {
    const GUEST_DATA_KEY: &'static str = "guest.data";
    // async fn mark_pageview(&mut self) {
    //     self.guest_data.pageviews += 1;
    //     self.guest_data.last_seen = OffsetDateTime::now_utc();
    //     Self::update_session(&self.session, &self.guest_data).await
    // }

    async fn update_session(session: &Session, guest_data: &GuestInfo) {
        session
            .insert(Self::GUEST_DATA_KEY, guest_data.clone())
            .await
            .unwrap()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Guest
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(
        req: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let _session = Session::from_request_parts(req, state).await?;

        let mut guest_data: GuestInfo = _session
            .get(Self::GUEST_DATA_KEY)
            .await
            .unwrap()
            .unwrap_or_default();

        guest_data.pageviews += 1;
        guest_data.last_seen = OffsetDateTime::now_utc();
        log::info!(
            "identified user id={} page_views={}",
            guest_data.user_id,
            guest_data.pageviews
        );
        Self::update_session(&_session, &guest_data).await;

        Ok(Self {
            _session,
            guest_data,
        })
    }
}

pub fn make_session_layer() -> SessionManagerLayer<SledStore> {
    use crate::database::config::SERVER_DATA_PATH;

    let sled = sled::open(format!("{SERVER_DATA_PATH}/sessions.sled")).unwrap();
    let session_store = SledStore::new(sled.open_tree("sessions").unwrap());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(666)));
    session_layer
}
