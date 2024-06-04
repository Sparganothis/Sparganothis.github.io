use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

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
    )
    .expect("bad css");
    use leptos_hotkeys::{provide_hotkeys_context, scopes, HotkeysContext};

    provide_meta_context();

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());
    use crate::page_1p::Game1P;
    use crate::page_2p::Game2P;
    use crate::page_replay::GameReplay;
    use crate::page_vs_cpu::GameCPU;
    view! {
        <div class=_style.get_class_name().to_string()>
            <Router>
                <nav>
                    <MainMenu />
                </nav>
                <main  _ref=main_ref>
                    // all our routes will appear inside <main>
                    <Routes>
                        <Route path="" view=Game1P> </Route>
                        <Route path="/vs_cpu" view=GameCPU></Route>
                        <Route path="/vs_net" view=Game2P> </Route>
                        <Route path="/replay" view=GameReplay> </Route>

                    </Routes>
                </main>
            </Router>
        </div>
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
        ]
    };
    view! {
        <ul class="menu_root">
            <For
                each=menu_entries
                key= |k| k.0
                children= |k| view!  {
                    <A href=k.0>
                    <h3 class="menu_item">
                    {k.1}
                    </h3>
                    </A>
                }
            />
        </ul>
    }
}
