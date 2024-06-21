use futures::StreamExt;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Title};
use leptos_router::*;
// use crate::error_template::ErrorTemplate;
use game::api::websocket::{WebsocketAPIMessageRaw, WhoAmI};
use leptonic::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{use_websocket, UseWebsocketReturn};

use crate::comp::game_board_mspaint::MsPaintGameBoard;

#[component]
pub fn AppRoot() -> impl IntoView {
    let _style = stylist::style!(
        nav {
            position: absolute;
            left: 1vmin;
            top: 1vmin;
            height: 97vmin;
            width: 19.2vmin;
            border: 1vmin solid black;
        }
        main {
            position: absolute;
            top: 1vmin;
            left: 19.85vmin;
            height: 98vmin;
        }
        main > div.main_left {
            position: absolute;
            top: 0vmin;
            width: 71.1vmin;
            left: 1.1vmin;
            height: 97vmin;
            border: 1vmin solid green;
        }
        main > div.main_right {
            position: absolute;
            top: 0vmin;
            width: 71.1vmin;
            left: 72.85vmin;
            height: 97vmin;
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

    use crate::websocket::demo_comp::*;
    use std::rc::Rc;

    let UseWebsocketReturn {
        ready_state,
        // message,
        message_bytes,
        // send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket("ws://localhost:3000/api/ws");

    let connected = move || ready_state.get() == ConnectionReadyState::Open;
    let mut ready_state_stream = ready_state.clone().to_stream();
    let ready_signal = create_rw_signal(false);

    let (tx, rx) = async_channel::bounded::<ConnectionReadyState>(1);
    spawn_local(async move {
        loop {
            let r = ready_state_stream.next().await;
            if let Some(r) = r {
                if r.eq(&ConnectionReadyState::Open) {
                    ready_signal.set(true);
                } else {
                    ready_signal.set(false);
                }
                if let Err(e) = tx.send(r).await {
                    log::warn!("error sending to ready stream...: {e:?}");
                } else {
                    log::info!("sent on stream: {:?}", r);
                }
            }
        }
    });

    let open_connection = move |_| {
        log::info!("websocket reopened.");
        open();
    };

    let close_connection = move |_| {
        log::info!("websocket closed intentionally.");
        close();
    };

    // let message =
    // let message = bincode::serialize(&message).unwrap();
    let api = WebsocketAPI {
        map: create_rw_signal(std::collections::HashMap::<_, _>::new()),
        sender: create_rw_signal(Rc::new(Box::new(send_bytes.clone()))),
        ready_state_stream: rx,
        ready_signal,
    };
    provide_context(api.clone());

    let api2 = api.clone();
    let send_byte_message = move |_| {
        let api2 = api2.clone();
        let _res = create_resource(
            || (),
            move |_| {
                let api2 = api2.clone();
                async move {
                    // log::info!("calling websocket api");
                    let r = call_websocket_api::<WhoAmI>(api2, ())
                        .expect("cannot obtain future")
                        .await;
                    // log::info!("got back response: {:?}", r);
                    r
                }
            },
        );
    };
    let mut recv_bytes_stream = message_bytes.to_stream();
    // let last_message_size = create_rw_signal(0);
    // let last_message_id = create_rw_signal(0);

    log::info!("console init");
    let api_spawn = api.clone();
    spawn_local(async move {
        log::info!("spawn local init");
        use futures::stream::StreamExt;
        loop {
            while let Some(Some(c)) = recv_bytes_stream.next().await {
                // last_message_size.set_untracked(c.len() as i32);
                // last_message_id.set_untracked(last_message_id.try_get_untracked().unwrap_or(0) % 999);
                log::debug!("websocket got {} bytes", c.len());
                match bincode::deserialize::<WebsocketAPIMessageRaw>(&c) {
                    Ok(msg) => {
                        // log::info!("recv message type={:?} len={}", msg._type, c.len(),);
                        accept_reply_message(&api_spawn.clone(), msg).await;
                        // let ctx = expect_context::<RwSignal<WebsocketAPI>>();
                        // log::info!("successfully got global context size={}!", ctx.get_untracked().map.len());
                    }
                    Err(e) => {
                        log::warn!("websocket deserialize error {:?}", e);
                    }
                }
            }
            log::info!("websocket reciever died.");
            // thread::sleep(std::time::Duration::from_millis(3));
        }
    });

    let status = move || {
        let st = ready_state.get().to_string();
        log::info!("websocket status: {}", st);
        st
    };

    use crate::comp::game_board_spectator::SpectatorGameBoard;
    use crate::page::page_1p::Game1PPage;
    use crate::page::page_2p::Game2PPage;
    use crate::page::page_user_profile::{MyAccountPage, UserProfilePage};
    use crate::page::page_vs_cpu::GameCPUPage;

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="FALLING BLOCKS"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        // <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        // <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="TOTRES"/>

        <Root default_theme=LeptonicTheme::default()>

            <div class=_style.get_class_name().to_string()>

                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors
                        .insert_with_default_key(
                            crate::error_template::AppError::NotFound,
                        );
                    view! {
                        <crate::error_template::ErrorTemplate outside_errors></crate::error_template::ErrorTemplate>
                    }
                }>
                    <nav>
                        <MainMenu/>
                        <div>
                            <p>"status: " {status}</p>

                            <button
                                on:click=send_byte_message
                                disabled=move || !connected()
                            >
                                "Send bytes"
                            </button>
                            <button on:click=open_connection disabled=connected>
                                "Open"
                            </button>
                            <button
                                on:click=close_connection
                                disabled=move || !connected()
                            >
                                "Close"
                            </button>

                            // <p>{sig}</p>
                            <p>
                                {move || {
                                    format!(
                                        "{:?} bytes",
                                        message_bytes.get().unwrap_or(vec![]).len(),
                                    )
                                }}

                            </p>
                        </div>
                    </nav>
                    <main _ref=main_ref>
                        // all our routes will appear inside <main>
                        <Routes>
                            <Route path="" view=Game1PPage/>
                            <Route path="/vs_cpu" view=GameCPUPage/>
                            <Route path="/vs_net" view=Game2PPage/>
                            <Route
                                path="/replay"
                                view=crate::page::page_replay_browser::GameReplayBrowserPage
                            />
                            <Route path="/account" view=MyAccountPage/>
                            <Route path="/ws_demo" view=SpectatorGameBoard/>
                            <Route path="/user/:user_id" view=UserProfilePage/>
                            <Route
                                path="/view-game/:game_id"
                                view=crate::page::page_replay_single::GameReplaySinglePage
                            />
                            <Route path="/mspaint" view=MsPaintGameBoard/>
                        </Routes>
                    </main>
                </Router>
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
            ("/mspaint", "mspaint"),
            ("/ws_demo", "ws_demo"),
        ]
    };
    // let git_version = create_resource(
    //     || (),
    //     |_| async move { crate::server::api::server_info::git_version().await },
    // );

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
    }
}
