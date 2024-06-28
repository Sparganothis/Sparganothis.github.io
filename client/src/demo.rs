
use leptos::*;
use leptonic::prelude::*;

use crate::audio3::play_sound;

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
        <Button on_click=move |_| play_sound("game_over")>game_over</Button>
        <Button on_click=move |_| play_sound("hard_drop")>hard_drop</Button>
        <Button on_click=move |_| play_sound("move")>move</Button>
        <Button on_click=move |_| play_sound("soft_drop")>soft_drop</Button>
        <Button on_click=move |_| play_sound("hold")>hold</Button>
        <Button on_click=move |_| play_sound("pre_123")>pre_123</Button>
        <Button on_click=move |_| play_sound("rotate")>rotate</Button>
    }
}