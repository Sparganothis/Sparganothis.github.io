
use leptos::*;
use leptonic::prelude::*;

use crate::audio3::play_audio;

#[component]
pub fn GameBoardFlexDemoPage() -> impl IntoView {
    view! {
        <div class="main_left">
            <AudioDemo/>
        </div>
    }
}

#[component]
pub fn AudioDemo() -> impl IntoView {
    view! {
        <Button on_click=move |_| play_audio("game_over") >game_over</Button>
        <Button on_click=move |_| play_audio("acccess_denied") >acccess_denied</Button>
        <Button on_click=move |_| play_audio("click") >click</Button>
        <Button on_click=move |_| play_audio("poker_chip") >poker_chip</Button>
        <Button on_click=move |_| play_audio("dunk") >dunk</Button>
    }
}