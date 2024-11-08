use futures::StreamExt;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Script, Title};
use leptos_router::*;
// use crate::error_template::ErrorTemplate;
use game::api::websocket::{GitVersion, WebsocketAPIMessageRaw, WhoAmI};
use leptonic::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{use_websocket, UseWebsocketReturn};

use crate::audio3::provide_audio_context;
use crate::demo::GameBoardFlexDemoPage;
use crate::comp::game_board_mspaint::{MsPaintPage, MsPaintPlayPage};
use crate::hotkey_context::provide_hotkeys_context2;
use crate::mobile_check::is_mobile_phone;
use crate::page::homepage::Homepage;
use crate::page::page_spectate::SpectateGamePage;
use crate::page::page_match::MatchPage;
use crate::page::page_1p_lobby::GameSoloLobbyPage;
use crate::page::settings::server_api::provide_user_setting;
use crate::page::settings::settings_page::MMySettingsPage;
use crate::page::you_are_phone::you_are_phone_view;
use crate::comp::websocket_error_display::WebsocketErrorDisplay;
use crate::websocket::demo_comp::call_api_sync_or_error;
use crate::comp::game_board_flex::FlexText;
use crate::comp::feedback_overlay::FeedbackButtonExpander;

#[component]
pub fn AppRoot() -> impl IntoView {
    if is_mobile_phone() {
        return you_are_phone_view();
    }
    let _style = stylist::style!(
        nav {
            // height: 100%;
            min-width: 11.5%;
            width: 13.5%;
            max-width:13.5%;
            border: 1vmin solid black;
        }
        main {
            height:100%;width:100%;  display:flex; flex-direction:row;
            border: 1vmin solid white;
            background-color:#ffffff8f;
        }
        main > div.main_left {
            width: 50%;
            // display:flex;align-items:center;
            // height: 100%;
            // border: 1vmin solid green;
            max-height:100%; overflow:scroll;

        }
        main > div.main_mid {
            width: 6%;
            // display:flex;align-items:center;
            // height: 100%;
            border: 1vmin solid transparent;
            aspect-ratio: 93/873;
        }
        main > div.main_right {
            width: 50%;
            // display:flex;align-items:center;
            // height: 100%;
            // border: 1vmin solid blue;
            max-height:100%; overflow:scroll;
        }
        .menu_root {
            padding: 0px;
            margin:0px;
            flex: 0 1 auto;
            display:flex;
            flex-direction:column;
            height:100%;
            width:100%;
        }
        .menu_item {
            container-type:size;
            margin: 0px;
            height:100%;
            width:100%;
            text-align: center;
            font-weight: normal;
            color: black;
            rotate: -8deg;
        }
        a {
            text-decoration: none;
        }
        a[aria-current="page"] .menu_item  {
            font-weight: bold;
            color: darkred;
        }
        .profile_view_container {
            color: black;
            margin: 10px;
            border: 8px dashed black;
            padding: 15px;
            // width: 50%;
        }
    )
    .expect("bad css");


    provide_meta_context();

    provide_audio_context();


    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context2(main_ref);

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
    // } = use_websocket("wss://ws.sparganothis.org/api/ws");
    } = use_websocket(include_str!("websocket.txt"));

    let connected = move || ready_state.get() == ConnectionReadyState::Open;
    let mut ready_state_stream = ready_state.clone().to_stream();
    let ready_signal = create_rw_signal(false);

    let (tx, rx) = async_broadcast::broadcast::<ConnectionReadyState>(1);
    spawn_local(async move {
        loop {
            let r = ready_state_stream.next().await;
            if let Some(r) = r {
                if r.eq(&ConnectionReadyState::Open) {
                    ready_signal.set(true);
                } else {
                    ready_signal.set(false);
                }
                if let Err(e) = tx.broadcast(r).await {
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

    let api = WebsocketAPI {
        map: create_rw_signal(std::collections::HashMap::<_, _>::new()),
        sender: create_rw_signal(Rc::new(Box::new(send_bytes.clone()))),
        ready_state_stream: rx.deactivate(),
        ready_signal,
        subscribe_game_callbacks: create_rw_signal(std::collections::HashMap::<_, _>::new()),
        error_msgs: create_rw_signal(Vec::<_>::new()),
    };
    provide_context(api.clone());

    let send_byte_message = move |_| {
        call_api_sync::<WhoAmI>((), move |r| {
            log::info!("WHO AMM I? {:?}", r);
        });
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
                // log::debug!("websocket got {} bytes", c.len());
                match bincode::deserialize::<WebsocketAPIMessageRaw>(&c) {
                    Ok(msg) => {
                        // log::info!("recv message type={:?} len={}", msg._type, c.len(),);
                        if msg.is_req {
                            if let Err(e) = accept_subscribe_notification(&api_spawn.clone(), msg).await {
                                log::warn!("error accepting subscribe notifgication: {:?}", e);
                            }
                        } else {
                            accept_reply_message(&api_spawn.clone(), msg).await;
                        }
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
        let st = ready_state.get();
        log::info!("websocket status: {}", st);
        let flextext = view! { <FlexText text={ move || { st.to_string() } }.into_signal() size_cqh=60.0/> };
        match st {
            ConnectionReadyState::Open => {
                view! { <h1 style="color:darkgreen;height:100%;width:100%;">{flextext}</h1> }.into_view()
            },
            _  => {
                view! { <h1 style="color:red;height:100%;width:100%;">{flextext}</h1> }.into_view()
            }
        }
    };

    provide_user_setting();

    use crate::page::page_1p_play::Game1PPage;
    use crate::page::page_2p_lobby::Game2LobbyPage;
    use crate::page::page_user_profile::{MyAccountPage, UserProfilePage};
    use crate::page::page_vs_cpu::GameCPUPage;
    use crate::page::page_about::AboutPage;

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="FALLING BLOCKS"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Script type_="module" src="/public/js/mobile_check.js"/>
        <Script type_="module" src="/public/js/audio.js"/>

        // <link rel="icon" type="" href="http://example.com/image.png" />
        // <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="xanthoides"/>

        <Root default_theme=LeptonicTheme::default()>
            // BACKGROUND IMAGE
            <div style="position:absolute; left: 0px; top: -5vh; width: 100vw; height: 100vh;   background-image: url('/public/favicon.png');   background-repeat: no-repeat;  background-size: cover;  opacity: 0.5; z-index:-999;"></div>
            // FEEDBACK BUTTON
            <FeedbackButtonExpander/>

            <div style="width:100%;height:100%; align-items:center;">

                <div
                    class=_style.get_class_name().to_string()
                    style=" max-width:100%; max-height:100%; aspect-ratio:16/10; display:flex; flex-direction:row; align-items:center; margin: auto;"
                >

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
                            <div style="display:flex; flex-direction:column;aspect-ratio: 182/1030; max-height: 100%; max-width:100%; min-height:80%; min-width:90%; margin: auto;">
                                <div style="height:60%; width: 100%;">
                                    <MainMenu/>
                                </div>
                                <div style="height:15%;width:100%;border-top:1vmin solid black;">
                                    <div style="height:43%; width:100%;">{status}</div>
                                    <div style="height:33%;width:100%;display:flex;flex-direction:row;container-type:size;">

                                        <Button
                                            style="height:100%; width:33%; font-size:40cqh; line-height:50cqh;"
                                            on_click=send_byte_message
                                            disabled=(move || !connected()).into_signal()
                                        >
                                            "Send"
                                        </Button>
                                        <Button
                                            on_click=open_connection
                                            disabled=connected.into_signal()
                                            style="height:100%; width:33%; font-size:40cqh;line-height:50cqh;"
                                        >
                                            "Open"
                                        </Button>
                                        <Button
                                            style="height:100%; width:33%; font-size:40cqh;line-height:50cqh;"
                                            on_click=close_connection
                                            disabled=(move || !connected()).into_signal()
                                        >
                                            "Close"
                                        // <p>{sig}</p>
                                        </Button>

                                    </div>

                                    <div style="height:23%; width:100%;">
                                        <FlexText
                                            text={
                                                move || {
                                                    format!(
                                                        "{:?} bytes",
                                                        message_bytes.get().unwrap_or(vec![]).len(),
                                                    )
                                                }
                                            }
                                                .into_signal()
                                            size_cqh=60.0
                                        />
                                    </div>

                                </div>

                                <div style="height:7%; width: 100%;">
                                    <WebsocketErrorDisplay/>
                                </div>

                                <a
                                    href="https://github.com/Sparganothis/Sparganothis.github.io"
                                    target="_blank"
                                    style="height:7%; width: 100%; display: flex;flex-direction:row; "
                                >
                                    <Icon
                                        icon=icondata::BsGithub
                                        width="100%"
                                        height="100%"
                                        style="width:20%"
                                    />
                                    <div style="width:80%; height:100%;">
                                        <FlexText text="GitHub" size_cqh=60.0/>
                                    </div>

                                </a>

                                <div style="height:15%;width:100%;">
                                    <VersionDisplayComp/>
                                </div>
                            </div>
                        </nav>
                        <main _ref=main_ref>
                            // all our routes will appear inside <main>
                            <Routes>
                                <Route path="" view=RootRedirectPage/>
                                <Route path="/home" view=Homepage/>
                                <Route path="/lobby/:game_type" view=GameSoloLobbyPage/>

                                <Route
                                    path="/play-game/:game_type/:game_id"
                                    view=Game1PPage
                                />
                                <Route path="/vs_cpu" view=GameCPUPage/>
                                <Route path="/vs_net" view=Game2LobbyPage/>
                                <Route
                                    path="/replay"
                                    view=crate::page::page_replay_browser::GameReplayBrowserPage
                                />
                                <Route path="/account" view=MyAccountPage/>
                                <Route path="/settings" view=MMySettingsPage/>
                                <Route
                                    path="/spectate-game/:game_id"
                                    view=SpectateGamePage
                                />
                                <Route path="/user/:user_id" view=UserProfilePage/>
                                <Route
                                    path="/view-game/:game_id"
                                    view=crate::page::page_replay_single::GameReplaySinglePage
                                />
                                <Route path="/match/:match_id" view=MatchPage/>
                                <Route path="/mspaint" view=MsPaintPage/>
                                <Route path="/edit-custom-game/:save_id" view=MsPaintPage/>
                                <Route
                                    path="/play-custom-game/:save_id"
                                    view=MsPaintPlayPage
                                />
                                <Route path="/demo" view=GameBoardFlexDemoPage/>
                                <Route path="/about" view=AboutPage/>
                                <Route path="/you-are-phone" view=you_are_phone_view/>

                            </Routes>
                        </main>
                    </Router>
                </div>

            </div>
        </Root>
    }.into_view()
}

#[component]
pub fn MainMenu() -> impl IntoView {
    let menu_entries = || {
        vec![
            ("/home", "home"),
            ("/lobby/solo", "solo"),
            ("/vs_cpu", "man vs car"),
            ("/vs_net", "1v1 online"),
            ("/replay", "replay"),
            ("/account", "account"),
            ("/settings", "settings"),
            ("/mspaint", "mspaint"),
            ("/demo", "DEMO"),
            ("/about", "about"),
        ]
    };

    let menu_cnt = menu_entries().len(); 
    let aspect_ratio = menu_cnt as f32 * ( 144.0 / 467.0);
    view! {
        <ul class="menu_root" style="display:flex;flex-directon:column;">
            <For
                each=menu_entries
                key=|k| k.0
                children=move |k| {
                    view! {
                        <A
                            href=k.0
                            class="menu_item"
                            attr:style=move || format!("aspect-ratio:{};", aspect_ratio)
                        >
                            // <div >
                            <FlexText text=k.1 size_cqh=40.0/>
                        // </div>
                        // <h3 class="menu_item">{k.1}</h3>
                        </A>
                    }
                }
            />

        </ul>
    }
}



#[component]
pub fn RootRedirectPage()->impl IntoView {
    let n = leptos_router::use_navigate();
    queue_microtask(move || {
        log::info!("REDIRECTING TO /home....");
        n("/home", NavigateOptions{replace:true, ..Default::default()});
    });
    view! { <p>Redirecting to <a href="/home">home</a></p> }
}

#[component]
pub fn VersionDisplayComp() -> impl IntoView {
    let client_version = crate::git_version::GIT_VERSION;
    let client_version = client_version.replace("\n", "").replace(" ", "");

    let game_version = game::git_version::GIT_VERSION;
    let game_version: String = game_version.replace("\n", "").replace(" ", "");
    
    let server_version = create_rw_signal("".to_string());
    call_api_sync_or_error::<GitVersion>((), move |v| {
        server_version.set(v.replace("\n", "").replace(" ", ""));
    }, move |err| {
        server_version.set(format!("error: {err}"));
    });
    let c2 = client_version.clone();
    let g2 = game_version.clone();
    let s = move || {
        format!(" Client: {}\n Server: {}\n Game: {}", c2, server_version.get(), g2)
    };

    let c2 = client_version.clone();
    let g2 = game_version.clone();
    let style = move || {
        let color = if server_version.get() == c2 && c2 == g2 {
            "color:blue;".to_string()
        } else {
            "color:red;".to_string()
        };
        format!("height:100%;width:100%;margin:0px;padding:0px; {color}")
    };
    view! {
        <code style="height:100%;width:100%;margin:0px;padding:0px;">
            <pre style=style>
                <FlexText text=s.into_signal() size_cqh=17.0/>
            </pre>
        </code>
    }
}