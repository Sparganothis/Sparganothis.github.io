use leptos::*;

use crate::comp::table_replay_games::AllGamesTable;
use crate::websocket::demo_comp::call_api_sync;
use game::api::user;
use game::api::websocket::{GetAllGamesArg, GetProfile, WhoAmI};
use leptonic::prelude::*;

#[component]
pub fn MyAccountPage() -> impl IntoView {

    let guest_id = create_rw_signal(None);
    call_api_sync::<WhoAmI>((), move |r| {
        guest_id.set(Some(r));
    });

    let user_profile = create_rw_signal(None);
    create_effect(move |_|{
        if let Some(guest_id) = guest_id.get() {
            call_api_sync::<GetProfile>(guest_id.user_id, move |r| {
                user_profile.set(Some(r));
            });
        }
    });

    let user_link = move || {
        if let (Some(g_id), Some(profile)) =
            (guest_id.get(), user_profile.get())
        {
            view! {
                <a href=format!("/user/{}", g_id.user_id)>
                    <UserProfileView _user_id=g_id.user_id p=profile/>
                </a>
            }
            .into_view()
        } else {
            view! { <p>-</p> }.into_view()
        }
    };

    view! {
        <h2>account</h2>
        <pre>{{ move || format!("guest_info: {:?}", guest_id.get()) }}</pre>

        <h2>profile</h2>
        <pre>{{ move || format!("user_profile: {:?}", user_profile.get()) }}</pre>
        <h3>{{ user_link }}</h3>
    }
}

#[component]
pub fn UserProfilePage() -> impl IntoView {
    let params = leptos_router::use_params_map();

    let user_uuid = create_rw_signal(None);
    create_effect(move |_| {
        if let Ok(uuid) = params.with(|params| {
            uuid::Uuid::parse_str(&params.get("user_id").cloned().unwrap_or_default())}) {
                user_uuid.set(Some(uuid))
            }
    });

    let user_profile = create_rw_signal(None);
    create_effect(move |_|{
        if let Some(guest_id) = user_uuid.get() {
            call_api_sync::<GetProfile>(guest_id, move |r| {
                user_profile.set(Some(r));
            });
        }
    });

    let profile_view = move || {
        if let (Some(user_id), Some(profile)) = (user_uuid.get(), user_profile.get()) {
            view! { <UserProfileView p=profile _user_id=user_id/> }.into_view()
        } else {
            view! { <p>profile not found!</p> }.into_view()
        }
    };

    view! { <div>{move || profile_view()}</div> }
}

#[component]
pub fn UserProfileView(_user_id: uuid::Uuid, p: user::UserProfile) -> impl IntoView {
    view! {
        <div class="profile_view_container">
            <h1>{{ &p.display_name }}</h1>
            <h3>user_id: {{ format!("{:?}", _user_id) }}</h3>

            <Tabs mount=Mount::WhenShown>
                <Tab
                    name="tab-best-user-games"
                    label="Best Games from $User".into_view()
                >
                    <AllGamesTable list_type=GetAllGamesArg::BestGamesForPlayer(
                        _user_id,
                    )/>
                </Tab>

                <Tab
                    name="tab-recent-user-games"
                    label="Recent Games from $User".into_view()
                >
                    <AllGamesTable list_type=GetAllGamesArg::RecentGamesForPlayer(
                        _user_id,
                    )/>
                </Tab>
            </Tabs>

            <code>
                <pre>{{ format!("{:#?}", &p) }}</pre>
            </code>

        </div>
    }
}
