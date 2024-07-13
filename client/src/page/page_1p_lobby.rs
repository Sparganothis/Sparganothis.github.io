use crate::{comp::{game_board::RandomOpponentGameBoard, game_board_flex::FlexText, menu_grid_view::MenuGridView}, websocket::demo_comp::call_api_sync};
use game::{api::{game_replay::GameId, websocket::CreateNewGameId}, random::GameSeed};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn GameSoloLobbyPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let redirect_to_new_game = Callback::new(move |_|{
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game-solo/{}", r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    let play_button = view! {
        <div style="width:100%;height:100%; container-type: size;">
            <h3
                style="font-size:80cqh; text-align: center;  cursor:pointer; "
                on:click=move |_| { redirect_to_new_game.call(()) }
            >
                PLAY
            </h3>
        </div>
    }
        .into_view();
       
    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            
            0 => {
                view! { <FlexText text="solo"/> }.into_view()
            },
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            6 => play_button.clone(),
            11 => {
                view! { <a href="/solo-lobby-40-lines" style="color:black;"> <FlexText text="40lines" size_cqh=60.0 /> </a> }.into_view()
            },
            12 => {
                view! { <a href="/solo-lobby-blitz" style="color:black;"> <FlexText text="blitz" size_cqh=60.0 /> </a> }.into_view()
            },
            _ => view!{            }.into_view()
            
            
        }
     }).collect();

    view! { <MenuGridView views/> }
}



#[component]
pub fn GameSoloLobby40LinesPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let redirect_to_new_game = Callback::new(move |_|{
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game-solo/{}", r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    let play_button = view! {
        <div style="width:100%;height:100%; container-type: size;">
            <h3
                style="font-size:80cqh; text-align: center;  cursor:pointer; "
                on:click=move |_| { redirect_to_new_game.call(()) }
            >
                PLAY
            </h3>
        </div>
    }
        .into_view();
       

    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            
            0 => {
                view! { <FlexText text="40lines"/> }.into_view()
            },
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            6 => play_button.clone(),
            11 => {
                view! { <a href="/solo-lobby-40-lines" style="color:black;"> <FlexText text="40lines" size_cqh=60.0 /> </a> }.into_view()
            },
            12 => {
                view! { <a href="/solo-lobby-blitz" style="color:black;"> <FlexText text="blitz" size_cqh=60.0 /> </a> }.into_view()
            },
            _ => view!{            }.into_view()
            
            
        }
     }).collect();

    view! { <MenuGridView views/> }
}




#[component]
pub fn GameSoloLobbyBLITZPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];
    let redirect_to_new_game = Callback::new(move |_|{
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game-solo/{}", r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    let play_button = view! {
        <div style="width:100%;height:100%; container-type: size;">
            <h3
                style="font-size:80cqh; text-align: center;  cursor:pointer; "
                on:click=move |_| { redirect_to_new_game.call(()) }
            >
                PLAY
            </h3>
        </div>
    }
        .into_view();
       

    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            
            0 => {
                view! { <FlexText text="BLITZ"/> }.into_view()
            },
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            6 => play_button.clone(),
            11 => {
                view! { <a href="/solo-lobby-40-lines" style="color:black;"> <FlexText text="40lines" size_cqh=60.0 /> </a> }.into_view()
            },
            12 => {
                view! { <a href="/solo-lobby-blitz" style="color:black;"> <FlexText text="blitz" size_cqh=60.0 /> </a> }.into_view()
            },
            _ => view!{            }.into_view()
            
            
        }
     }).collect();

    view! { <MenuGridView views/> }
}


