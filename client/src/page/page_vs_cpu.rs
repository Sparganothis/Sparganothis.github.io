use game::random::GameSeed;

use crate::comp::{game_board::RandomOpponentGameBoard, menu_grid_view::MenuGridView};
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

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            5 =>             youtube_video.clone(),

            8 =>view! { <RandomOpponentGameBoard seed=seed/> }.into_view(),
            _ => {                view!{                }.into_view()            }
        }
     }).collect();

    view! { <MenuGridView views/> }
}
