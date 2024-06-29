use leptos::*;

use crate::comp::table_replay_games::AllGamesTable;
use crate::websocket::demo_comp::call_api_sync;
use game::api::user::{self, GuestInfo};
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


    let user_settings = view! {
        <div>
            <Show
                when=move || user_profile.get().is_some() && guest_id.get().is_some()
                fallback=move || view! {}
            >

                {move || {
                    let user_profile = user_profile.get().unwrap();
                    let guest_id = guest_id.get().unwrap();
                    view! { <PersonalAccountSettingsForm user_profile guest_id/> }
                }}

            </Show>

        </div>
    };

    view! {
        <div class="main_left" style="width:95vmin">
            {{ user_settings }}
        </div>
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

            <Tabs mount=Mount::Once>
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

#[component]
pub fn PersonalAccountSettingsForm(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {
    use leptonic::prelude::*;

    // let (checked, set_checked) = create_signal(false);

    let (state, set_state) = create_signal(false);
    let (hsv, set_hsv) = create_signal(HSV::new());

    let user_info_str=  format!("user_profile: {:#?}", user_profile);
    let guest_id_str = format!("guest_info: {:#?}", guest_id);
    let signal_str = create_rw_signal((guest_id_str, user_info_str));


    let guest_id2=guest_id.clone();
    let user_profile2=user_profile.clone();
    let user_link = move || {
        let guest_id2=guest_id2.clone();
        let user_profile2=user_profile2.clone();
        view! {
            <a href=format!("/user/{}", guest_id2.user_id)>
                <UserProfileView _user_id=guest_id2.user_id p=user_profile2/>
            </a>
        }
        .into_view()
    };
    let user_link = create_rw_signal(user_link());
    let (value_menu_mmusic , set_value_menu_music) = create_signal(0.0);
    view! {
        <Tabs mount=Mount::Once>
            <Tab name="account" label="My Account".into_view()>
                <div style="width: 100%; padding: 1vh; margin: 1vh;">
                    <h2>account</h2>
                    <pre>{move || signal_str.get().0}</pre>

                    <h2>profile</h2>
                    <pre>{move || signal_str.get().1}</pre>
                </div>
            </Tab>

            <Tab name="profile" label="Game Profile".into_view()>
                <div style="width: 100%; padding: 1vh; margin: 1vh;">
                    {move || user_link.get()}
                </div>
            </Tab>

            <Tab name="sound" label="Sound".into_view()>
                <table>
                    <tr>
                        <td>
                            <Toggle state=state set_state=set_state/>
                        </td>
                        <td>
                            <h3>Menu Music</h3>
                        </td>
                        <td>
                            <h3>
                                {move || {
                                    (if state.get() { "ON" } else { "OFF" }).to_string()
                                }}

                            </h3>
                        </td>
                    </tr>
                    <tr>
                        <td style="width:20vmin;">
                            <Slider
                                min=0.0
                                max=100.0
                                step=1.0
                                value=value_menu_mmusic
                                set_value=set_value_menu_music
                                value_display=move |v| format!("{v:.0}")
                            />
                        </td>
                        <td>
                            <h3>Menu Music Volume</h3>
                        </td>
                        <td>{move || format!("{}", value_menu_mmusic.get())}</td>
                    </tr>
                    <tr>
                        <td>
                            <Toggle state=state set_state=set_state/>
                        </td>
                        <td>
                            <h3>Menu Music</h3>
                        </td>
                    </tr>
                </table>
            </Tab>

            <Tab name="controls" label="Controls".into_view()>
                <table>
                    <tr>
                        <td>
                            <Toggle state=state set_state=set_state/>
                        </td>
                        <td>
                            <h3>I Have ADHD</h3>
                        </td>
                    </tr>
                </table>
            </Tab>

            <Tab name="theme" label="Themes".into_view()>
                <div style="width:50%; height: 20%">
                    <ColorPicker hsv=hsv set_hsv=set_hsv/>
                </div>
            </Tab>
        </Tabs>
    }
}
   