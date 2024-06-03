use crate::{
    game_board::GameBoard,
    tet::{Tet, TetAction},
};
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
            width: 77vmin;
            height: 98vmin;
            border: 1vmin solid green;
        }
        main > div.main_right {
            position: absolute;
            top: 0vmin;
            width: 77vmin;
            left: 78.85vmin;
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

    view! {
        <div class=_style.get_class_name().to_string()>
            <Router>
                <nav>
                    <MainMenu />
                </nav>
                <main  _ref=main_ref>
                    // all our routes will appear inside <main>
                    <Routes>
                        <Route path="" view=|| {
                            view!{
                                <div class="main_left">
                                    <GameBoard/>
                                </div>
                            }
                        }> </Route>

                        <Route path="/vs_cpu" view=|| {
                            view!{
                                <div class="main_left">
                                    <GameBoard/>
                                </div>
                                <div class="main_right">
                                    <GameBoard/>
                                </div>
                            }
                        }> </Route>


                        <Route path="/vs_net" view=|| {
                            view!{
                                <p>todo</p>
                                <SomeComponent/>
                            }
                        }> </Route>



                    </Routes>
                </main>
            </Router>
        </div>
    }
}

use leptos_hotkeys::use_hotkeys;
#[component]
pub fn SomeComponent() -> impl IntoView {
    let (get_act, set_act) = create_signal(TetAction::Nothing);

    use_hotkeys!(("arrowup,keyx,ControlLeft,ControlRight") => move |_| {
        logging::log!("up has been pressed");
        set_act.update(|c| *c=TetAction::RotateRight);
    });

    use_hotkeys!(("arrowdown") => move |_| {
        logging::log!("down has been pressed");
        set_act.update(|c| *c = TetAction::SoftDrop);
    });

    use_hotkeys!(("Space") => move |_| {
        logging::log!("space has been pressed");
        set_act.update(|c| *c = TetAction::HardDrop);
    });

    use_hotkeys!(("KeyC,ShiftLeft,ShiftRight") => move |_| {
        logging::log!("C has been pressed");
        set_act.update(|c| *c = TetAction::Hold);
    });

    use_hotkeys!(("KeyZ") => move |_| {
        logging::log!("Z has been pressed");
        set_act.update(|c| *c = TetAction::RotateLeft);
    });

    use_hotkeys!(("ArrowLeft") => move |_| {
        logging::log!("Left has been pressed");
        set_act.update(|c| *c = TetAction::MoveLeft);
    });

    use_hotkeys!(("ArrowRight") => move |_| {
        logging::log!("Right has been pressed");
        set_act.update(|c| *c = TetAction::MoveRight);
    });

    view! { <p>Num Respects: {move || format!("{:?}", get_act())}</p> }
}

#[component]
pub fn MainMenu() -> impl IntoView {
    let menu_entries = || {
        vec![
            ("/", "home"),
            ("/vs_cpu", "1v1 cpu"),
            ("/vs_net", "1v1 online"),
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
