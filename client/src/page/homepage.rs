
use game::api::websocket::{GetAllGames, GetAllGamesArg};
// use game::tet::GameState;
use leptos::*;

use crate::{page::page_replay_single::GameReplayBoardMini, websocket::demo_comp::call_api_sync};

#[component]
pub fn Homepage()-> impl IntoView{

    let best_gameid = create_rw_signal(None);
    call_api_sync::<GetAllGames>(GetAllGamesArg::BestGames, move |v: Vec<_>| {
            let game_id = v.get(0).clone();
            if let Some((a, _b)) = game_id {
                best_gameid.set(Some(*a));
            }
    });

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            0 => {
                view! { <h1>todo</h1> }.into_view()
            },
            8 =>{
                (move || {
                    match best_gameid.get() {
                        Some(game_id) => view! { <GameReplayBoardMini game_id/> }.into_view(),
                        None => view!{}.into_view(),
                    }
                }).into_view()
            },
            9 => {
                view! {
                    <a href="/solo">
                        <h3>"SOLO"</h3>
                    </a>
                }.into_view()
            },
            _ => {
                view!{

                }.into_view()
            }
        }


     }).collect();

    view! { <MenuGridView views/> }
}


#[component]
pub fn MenuGridView(views:Vec<View>) -> impl IntoView {
    view! {
        <div class="main_left">
            <div style="border:solid purple 1px;height:7%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:20%;height:100%;">
                    {views.get(0)}
                </div>
                <div style="border:solid yellow 1px;width:20%;height:100%;">
                    {views.get(1)}
                </div>
                <div style="border:solid green 1px;width:20%;height:100%;">
                    {views.get(2)}
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                    {views.get(3)}
                </div>
                <div style="border:solid blue 1px;width:20%;height:100%;">
                    {views.get(4)}
                </div>
            </div>
            <div style="border:solid red 1px;height: 48%;flex-direction: row;display: flex;">
                <div style="border:solid blue 1px;width:50%;height:100%;flex-direction: column;display: flex;">
                    <div style="border:solid purple 1px;width:100%;height:50%;">
                        {views.get(5)}
                    </div>
                    <div style="border:solid green 1px;width:100%;height:25%;">
                        {views.get(6)}
                    </div>
                    <div style="border:solid red 1px;width:100%;height:25%;">
                        {views.get(7)}
                    </div>
                </div>
                <div style="border:solid green 1px;width:50%;height:100%;">
                    {views.get(8)}
                </div>
            </div>
            <div style="border:solid yellow 1px;height:10%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                    {views.get(9)}
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                    {views.get(10)}
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                    {views.get(11)}
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                    {views.get(12)}
                </div>

            </div>
            <div style="border:solid green 1px;height:35%;flex-direction: row;display: flex;">
                <div style="border:solid red 1px;width:25%;height:100%;">
                    {views.get(13)}
                </div>
                <div style="border:solid yellow 1px;width:25%;height:100%;">
                    {views.get(14)}
                </div>
                <div style="border:solid green 1px;width:25%;height:100%;">
                    {views.get(15)}
                </div>
                <div style="border:solid blue 1px;width:25%;height:100%;">
                    {views.get(16)}
                </div>

            </div>
        </div>
    }
}

