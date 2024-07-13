use crate::{comp::{game_board_flex::FlexText, menu_grid_view::MenuGridView}, websocket::demo_comp::call_api_sync};
use game::{api::{game_match::GameMatchType, game_replay::GameId, table_paginate::TablePaginateDirection, websocket::{CreateNewGameId, GetAllGames, GetAllGamesArg}}, random::GameSeed};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

use super::page_replay_single::GameReplayBoardMini;

#[component]
pub fn GameSoloLobbyPage() -> impl IntoView {
    let params = leptos_router::use_params_map();
    let _view = move || {
        let p = params.with(|params| params.get("game_type").cloned()).unwrap_or("".to_string());
        
        if let Ok(match_type) = GameMatchType::from_url(&p) {
            view!{<LobbyDisplay match_type/>}.into_view()
        } else {
            view!{<h1> Bad match type! </h1>}.into_view()
        }
    };
    _view.into_view()
}


#[component]
pub fn LobbyDisplay(match_type:Option<GameMatchType>, ) -> impl IntoView {
    let match_type_ = match_type.clone();

    let link_to_lobby = move |_opt:Option<GameMatchType>| {
        let text = GameMatchType::to_url(&_opt);
        view! {
             <a 
             href=format!("/lobby/{text}")
              style="color:black;">
               <FlexText 
               text 
               size_cqh=60.0 />
                </a> 
            }.into_view()
    };
    let views:Vec<_> = {0..20}.into_iter().map(move |x|{
        match x{
            0 => view!{
                <GameModeTitleDisplay match_type=match_type_.clone()/>
            },
            8 => view!{<BestMiniReplayForGameMode match_type=match_type_.clone()/>}.into_view(),
            5 => view! {<GameModeDescription match_type=match_type_.clone()/>}.into_view(),
            13 => view! {<GameModeOptions match_type=match_type_.clone()/>}.into_view(),
            6 => view! {<GameModeStartButton match_type=match_type_.clone()/>}.into_view(),
            10 => {link_to_lobby(None)},
            11 => {link_to_lobby(Some(GameMatchType::_40lines))},
            12 => {link_to_lobby(Some(GameMatchType::blitz))},
            _ => view!{ }.into_view()
        }
     }).collect();

    view! {
         <MenuGridView views/>
         
        <div class="main_right">
            <GameModeLeaderboard match_type=match_type.clone()/>
        </div>
     }

}

#[component]
pub fn BestMiniReplayForGameMode(match_type:Option<GameMatchType>, ) -> impl IntoView
{
    let best_gameid = create_rw_signal(None);
    call_api_sync::<GetAllGames>((GetAllGamesArg::BestGames, TablePaginateDirection::<GameId>::InitialPage
    ), move |v: Vec<_>| {
            let game_id = v.get(0).clone();
            if let Some((a, _b)) = game_id {
                best_gameid.set(Some(*a));
            }
    });

    (move || {
        match best_gameid.get() {
            Some(game_id) => view! { <GameReplayBoardMini game_id/> }.into_view(),
            None => view!{}.into_view(),
        }
    }).into_view()
}


#[component]
pub fn GameModeDescription(match_type:Option<GameMatchType>, ) -> impl IntoView {

    let text = match match_type {
        None => "Resumable session at zero speed - play forever",
        Some(GameMatchType::_40lines) => "CLEAR 40 LINES SHORTEST TIME  GOTTA GO FAST",
        Some(GameMatchType::blitz) => "GET BIG SCORE IN 2 MIN! LEVEL UP!",
        _ => "PRPOGRAMMER ERROR: DESCRIPTION NOT FOUND"
    };
    view!{
        <FlexText text size_cqh=9.0/>
    }
}


#[component]
pub fn GameModePersonalBest(match_type:Option<GameMatchType>, ) -> impl IntoView {   format!{     "GameModePersonalBest {:?}",match_type}.into_view()}

#[component]
pub fn GameModeLeaderboard(match_type:Option<GameMatchType>, ) -> impl IntoView {   format!{     "GameModeLeaderboard {:?}",match_type}.into_view()}

#[component]
pub fn GameModeStartButton(match_type:Option<GameMatchType>, ) -> impl IntoView { 
    let match_type_ = match_type.clone();
    let redirect_to_new_game = Callback::new(move |_|{
        let match_type_ = match_type_.clone();
        let navigate = use_navigate();
         call_api_sync::<CreateNewGameId>((), move |r:GameId| {
            let new_url = format!("/play-game/{}/{}", GameMatchType::to_url(&match_type_), r.to_url());
            navigate(&new_url, NavigateOptions::default());
         });        
    });
    
    view!{
        <div style="width:100%;height:100%; container-type: size;">
            <h3
                style="font-size:80cqh; text-align: center;  cursor:pointer; "
                on:click=move |_| { redirect_to_new_game.call(()) }
            >
                PLAY
            </h3>
        </div>
    }
        .into_view()
}

#[component]
pub fn GameModeOptions(match_type:Option<GameMatchType>, ) -> impl IntoView {   format!{     "GameModeOptions {:?}",match_type}.into_view()}

#[component]
pub fn GameModeTitleDisplay(match_type:Option<GameMatchType>, ) -> impl IntoView {  

    let text={match match_type {
        Some(r) => match r {
            GameMatchType::_40lines => "40lines",
            GameMatchType::blitz => "blitz",
            _ => "???",
        },
        None => {
            "solo"
        },
    }};
    view!{
        <FlexText text size_cqh=60.0/>
    }
}

