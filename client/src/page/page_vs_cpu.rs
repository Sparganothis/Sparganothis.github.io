use game::{api::{game_match::GameMatchType, websocket::StartMatch}, random::GameSeed};
use leptos_router::{use_navigate, NavigateOptions};

use crate::{comp::{game_board::RandomOpponentGameBoard, menu_grid_view::MenuGridView}, websocket::demo_comp:: call_api_sync_or_error};
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let youtube_video = view! {
        <iframe
            width="100%"
            height="100%"
            src="https://www.youtube-nocookie.com/embed/DrO9ySwbTjo"
            title="YouTube video player"
            frameborder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            referrerpolicy="strict-origin-when-cross-origin"
            allowfullscreen
        ></iframe>
    }.into_view();

    
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
                style="font-size:40cqh; text-align: center;"
                on:click=move |_| { obtain_new_match_id.call(bot_name.clone()) }
            >
                PLAY vs. BOT:  {bot_name2.clone()}
            </h3>
        </div>
    }
        .into_view()};

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            5 =>             youtube_video.clone(),

            6 => play_button("random".to_string()).clone(),
            7 => play_button("wordpress".to_string()).clone(),
            7 => view! { {move || error_display.get()} }.into_view(),
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }.into_view(),
            _ => {                view!{                }.into_view()            },
        }
     }).collect();

    view! { <MenuGridView views/> }
}
