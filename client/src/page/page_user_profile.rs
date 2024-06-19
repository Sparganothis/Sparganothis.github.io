use leptos::*;

use crate::websocket::demo_comp::{call_websocket_api, WebsocketAPI};
use game::api::user;
use game::api::websocket::{GetProfile, WhoAmI};


#[component]
#[allow(unused_variables)]
pub fn MyAccountPage() -> impl IntoView {
    let _api: WebsocketAPI = expect_context();
    let guest_id = create_resource(
        || (),
        move |_| {
            let api_bis = _api.clone();
            async move {
                // log::info!("calling websocket api");
                let r = call_websocket_api::<WhoAmI>(api_bis, ())
                    .expect("cannot obtain future")
                    .await;
                // log::info!("got back response: {:?}", r);
                r
            }
        },
    );

    let _api2: WebsocketAPI = expect_context();
    let user_profile = create_resource(
        move || guest_id.get(),
        move |_guest_id| {
            let api2_bis = _api2.clone();
            async move {
                if let Some(Ok(_guest_id)) = _guest_id {
                    // log::info!("calling websocket api");
                    let r: Result<user::UserProfile, String> =
                        call_websocket_api::<GetProfile>(api2_bis, _guest_id.user_id)
                            .expect("cannot obtain future")
                            .await;
                    // log::info!("got back response: {:?}", r);
                    r
                } else {
                    Err("fmm loading...".to_string())
                }
            }
        },
    );

    let user_link = move || {
        if let (Some(Ok(g_id)), Some(Ok(profile))) = (guest_id.get(), user_profile.get()) {
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
    let api2: WebsocketAPI = expect_context();
    let params = leptos_router::use_params_map();
    let _uuid = params
        .with(|params| uuid::Uuid::parse_str(&params.get("user_id").cloned().unwrap_or_default()));
    let (get_id, _) = create_signal(_uuid);

    let user_profile = create_resource(
        move || get_id.get(),
        move |_guest_id| {
            let api2 = api2.clone();
            async move {
                if let Ok(_guest_id) = _guest_id {
                    // log::info!("calling websocket api");
                    let r: Result<user::UserProfile, String> =
                        call_websocket_api::<GetProfile>(api2, _guest_id)
                            .expect("cannot obtain future")
                            .await;
                    // log::info!("got back response: {:?}", r);
                    r
                } else {
                    Err("fmm loading...".to_string())
                }
            }
        },
    );

    let profile_view = move || {
        if let (Ok(user_id), Some(Ok(profile))) = (get_id.get(), user_profile.get()) {
            view! { <UserProfileView p=profile _user_id=user_id/> }.into_view()
        } else {
            view! { <p>profile not found!</p> }.into_view()
        }
    };

        view! {
            <div>{
                move || profile_view()}
            </div> 
        }
}

#[component]
pub fn UserProfileView(_user_id: uuid::Uuid, p: user::UserProfile) -> impl IntoView {
    view! {
        <div class="profile_view_container">
            <h1>{{ &p.display_name }}</h1>
            <h3>user_id: {{ format!("{:?}", _user_id) }}</h3>
            <pre>{{ format!("User Profile: {:?}", &p) }}</pre>
        </div>
    }
}
