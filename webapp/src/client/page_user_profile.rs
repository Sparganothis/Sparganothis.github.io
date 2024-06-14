use leptos::*;

use crate::server::api::user;

#[component]
pub fn MyAccountPage() -> impl IntoView {
    let guest_id = create_resource(|| (), |_| async move { user::who_am_i().await });

    let user_profile = create_resource(
        move || guest_id.get(),
        |g| async move {
            if let Some(Ok(guest_info)) = g {
                user::get_profile(guest_info.user_id).await
            } else {
                Err(ServerFnError::new("cannot get user profile"))
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
    let params = leptos_router::use_params_map();
    let _uuid = params
        .with(|params| uuid::Uuid::parse_str(&params.get("user_id").cloned().unwrap_or_default()));
    let (get_id, _) = create_signal(_uuid);

    let profile = create_resource(
        move || get_id.get(),
        |uuid| async move {
            if let Ok(uuid) = uuid {
                user::get_profile(uuid).await
            } else {
                Err(ServerFnError::new("no profile"))
            }
        },
    );
    let profile_view = move || {
        if let (Ok(user_id), Some(Ok(profile))) = (get_id.get(), profile.get()) {
            view! { <UserProfileView p=profile _user_id=user_id/> }.into_view()
        } else {
            view! { <p>profile not found!</p> }.into_view()
        }
    };

    view! { <div>{{ move || profile_view() }}</div> }
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
