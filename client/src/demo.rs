
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
        <Button on_click=move |_| play_audio("game_over")>game_over</Button>
        <Button on_click=move |_| play_audio("hard_drop")>hard_drop</Button>
        <Button on_click=move |_| play_audio("move")>move</Button>
        <Button on_click=move |_| play_audio("soft_drop")>soft_drop</Button>
        <Button on_click=move |_| play_audio("hold")>hold</Button>
        <Button on_click=move |_| play_audio("pre_123")>pre_123</Button>
        <Button on_click=move |_| play_audio("rotate")>rotate</Button>
    }
}