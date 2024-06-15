use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;
// use crate::error_template::ErrorTemplate;
use leptonic::prelude::*;

#[component]
pub fn AppRoot() -> impl IntoView {
    let _style = stylist::style!(
        nav {
            position: absolute;
            left: 0vmin;
            top: 0vmin;
            height: 98vmin;
            width: 18vmin;
            border: 1vmin solid black;
        }
        main {
            position: absolute;
            top: 0vmin;
            left: 19.85vmin;
            height: 100vmin;
        }
        main > div.main_left {
            position: absolute;
            top: 0vmin;
            width: 70.1vmin;
            height: 98vmin;
            border: 1vmin solid green;
        }
        main > div.main_right {
            position: absolute;
            top: 0vmin;
            width: 70.1vmin;
            left: 71.85vmin;
            height: 98vmin;
            border: 1vmin solid blue;
        }
        .menu_root {
            padding: 0px;
        }
        .menu_item {
            margin: 0px;
            height: 6vmin;
            text-align: center;
            line-height: 6vmin;
            font-size: 3vmin;
            font-weight: normal;
            color: black;
            rotate: -11deg;
        }
        a {
            text-decoration: none;
        }
        a[aria-current="page"] > .menu_item  {
            font-weight: bold;
            color: darkred;
            border: 0.5vmin darkred solid;
            margin: 0.5vmin;
            height: 5vmin;
            line-height: 5vmin;
        }
        .profile_view_container {
            color: black;
            margin: 10px;
            border: 8px dashed black;
            padding: 15px;
        }
    )
    .expect("bad css");
    use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};

    provide_meta_context();

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());
    use super::game_board_spectator::SpectatorGameBoard;
    use super::page_1p::Game1PPage;
    use super::page_2p::Game2PPage;
    use super::page_replay::GameReplayPage;
    use super::page_user_profile::{MyAccountPage, UserProfilePage};
    use super::page_vs_cpu::GameCPUPage;

    view! {
        <Title text="FALLING BLOCK GAME"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>
        <Meta name="charset" content="UTF-8"/>
        <Stylesheet id="leptos" href="/leptonic-ciordit.css"/>
        <Stylesheet id="leptos" href="/table-bootstrap.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/css/bootstrap.min.css"/>


        <Root default_theme=LeptonicTheme::default()>
        <div class=_style.get_class_name().to_string()>
            // <Transition fallback=move || view! {<p>"Loading..."</p> }>
            // <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
            <Router>
                <nav>
                    <MainMenu/>
                </nav>
                <main _ref=main_ref>
                    // all our routes will appear inside <main>
                    <Routes>
                        <Route path="" view=Game1PPage/>
                        <Route path="/vs_cpu" view=GameCPUPage/>
                        <Route path="/vs_net" view=Game2PPage/>
                        <Route path="/replay" view=GameReplayPage/>
                        <Route path="/account" view=MyAccountPage/>
                        <Route path="/ws_demo" view=SpectatorGameBoard/>
                        <Route path="/user/:user_id" view=UserProfilePage/>
                        <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>

        // </ErrorBoundary>
        // </Transition>
        </div>

        </Root>
    }
}

#[component]
pub fn MainMenu() -> impl IntoView {
    let menu_entries = || {
        vec![
            ("/", "home"),
            ("/vs_cpu", "1v1 cpu"),
            ("/vs_net", "1v1 online"),
            ("/replay", "replay"),
            ("/account", "account"),
            ("/settings", "settings"),
            ("/about", "about"),
            ("/credits", "credits"),
            ("/sse_demo", "sse_demo"),
            ("/ws_demo", "ws_demo"),
        ]
    };
    let git_version = create_resource(
        || (),
        |_| async move { crate::server::api::server_info::git_version().await },
    );

    view! {
        <ul class="menu_root">
            <For
                each=menu_entries
                key=|k| k.0
                children=|k| {
                    view! {
                        <A href=k.0>
                            <h3 class="menu_item">{k.1}</h3>
                        </A>
                    }
                }
            />

        </ul>
        <p>{{ git_version }}</p>
    }
}
