use game::random::GameSeed;

use crate::comp::game_board::RandomOpponentGameBoard;
use leptos::*;
#[component]
pub fn GameCPUPage() -> impl IntoView {
    let seed: GameSeed = [0; 32];

    view! {
        <div class="main_left">
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
        </div>
        <div class="main_right">
            <RandomOpponentGameBoard seed=seed/>
        </div>
    }
}
