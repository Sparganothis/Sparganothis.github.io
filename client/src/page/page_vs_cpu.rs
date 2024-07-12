use game::{api::{game_match::GameMatchType, websocket::StartMatch}, random::GameSeed};
use leptos_router::{use_navigate, NavigateOptions};

use crate::{comp::{game_board::RandomOpponentGameBoard, game_board_flex::FlexText, menu_grid_view::MenuGridView}, mobile_check::is_mobile_phone, websocket::demo_comp:: call_api_sync_or_error};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let youtube_video = move || {
        let is_clicked = create_rw_signal(false);
        view! {
            <Show
                when=move || is_clicked.get()
                fallback=move || {
                    view! {
                        <img
                            on:click=move |_| { is_clicked.set(true) }
                            style="cursor:pointer;"
                            src="/public/img/thumb-youtube.png"
                            width="100%"
                            height="100%"
                        />
                    }
                        .into_view()
                }
            >

                <iframe
                    width="100%"
                    height="100%"
                    src="https://www.youtube-nocookie.com/embed/DrO9ySwbTjo"
                    title="YouTube video player"
                    frameborder="0"
                    allow="picture-in-picture"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen
                ></iframe>
            </Show>
        }.into_view()};

    
    let error_display = create_rw_signal("".to_string());

    let match_id_signal = create_rw_signal(None);

    let obtain_new_match_id: Callback<String> = Callback::new(move |bot_name| {
        call_api_sync_or_error::<StartMatch>(GameMatchType::ManVsCar(bot_name), move |r| {
            match_id_signal.set(Some(r));
            
        }, move |err_str| {
            error_display.set(err_str);
        });
    });
    
    create_effect(move |_| {
        if let Some(newgame) = match_id_signal.get() {
            let match_id = newgame.0;
            let url = format!("/match/{}", match_id);
            let navigate = use_navigate();
            navigate(&url, NavigateOptions::default());
        }
    });

    let play_button = move |bot_name: String|{ 
        let bot_name2 = bot_name.clone();
        view! {
            <div style="width:100%;height:100%; container-type: size;">
                <h3
                    style="font-size:40cqh; text-align: center; cursor:pointer;"
                    on:click=move |_| { obtain_new_match_id.call(bot_name.clone()) }
                >
                    PLAY vs. BOT:
                    <b>{bot_name2.clone()}</b>
                </h3>
            </div>
        }
        .into_view()};

    
    let play_button =move |bot_name: String|{ 
        if is_mobile_phone() {
            view! { <p style="color:red">You are phone. Plz use PC to play.</p> }.into_view()
        } else {
            play_button(bot_name)
        }
    };

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            
            0 => {
                view! { <FlexText text="car" /> }.into_view()
            },
            5 =>             youtube_video.clone().into_view(),

            6 => play_button("random".to_string()).clone(),
            7 => play_button("wordpress".to_string()).clone(),
            9 => view! { {move || error_display.get()} }.into_view(),
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }.into_view(),
            _ => {                view!{                }.into_view()            },
        }
     }).collect();

    view! { <MenuGridView views/> }
}
