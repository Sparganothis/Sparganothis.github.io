use game::bot::get_bot_from_id;
use leptos::*;

use crate::comp::table_replay_games::AllGamesTable;
use crate::page::settings::sound::SoundSettingsTab;
use crate::page::settings::themes::ThemeSettingsTab;
use crate::page::settings::controls::ControlsSettingsTab;
use crate::websocket::demo_comp::call_api_sync;
use game::api::user::{self, GuestInfo, UserProfile};
use game::api::websocket::{GetAllGamesArg, GetProfile, WhoAmI};
use leptonic::prelude::*;

#[component]
pub fn MMySettingsPage() -> impl IntoView {

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
    };

    view! { <div class="main_left">{{ user_settings }}</div> }
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

            if let Ok(bot_name) = get_bot_from_id(guest_id) {
                user_profile.set(Some(UserProfile { 
                    display_name: format!("BOT {}", bot_name) }));
            } else {

                call_api_sync::<GetProfile>(guest_id, move |r| {
                    user_profile.set(Some(r));
                });
            }
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


    let seound_settings_tab = create_rw_signal(view! { <SoundSettingsTab user_profile=user_profile.clone() guest_id=guest_id.clone()/> });

    let control_settings_tab = create_rw_signal(view! {
        <ControlsSettingsTab
            user_profile=user_profile.clone()
            guest_id=guest_id.clone()
        />
    });

    let theme_settings = create_rw_signal(view! { <ThemeSettingsTab user_profile=user_profile.clone() guest_id=guest_id.clone()/> });
    
    

    view! {
        <Tabs mount=Mount::Once>
            <Tab name="sound" label="Sound".into_view()>
                {seound_settings_tab.get_untracked()}
            </Tab>

            <Tab name="controls" label="Controls".into_view()>
                {control_settings_tab.get_untracked()}
            </Tab>

            <Tab name="theme" label="Themes".into_view()>
                {theme_settings.get_untracked()}

            </Tab>
        </Tabs>
    }
}
   
   
#[component]
pub fn MySettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {

    let user_info_str=  format!("user_profile: {:#?}", user_profile);
    let guest_id_str = format!("guest_info: {:#?}", guest_id);
    let signal_str = create_rw_signal((guest_id_str, user_info_str));

    view! {
        <div style="width: 100%; padding: 1vh; margin: 1vh;">
            <h2>account</h2>
            <pre>{move || signal_str.get().0}</pre>

            <h2>profile</h2>
            <pre>{move || signal_str.get().1}</pre>
        </div>
    }

}