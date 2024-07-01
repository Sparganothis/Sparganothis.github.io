
use leptos::*;
use leptonic::prelude::*;

use crate::audio3::play_sound_effect;

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
        <Button on_click=move |_| play_sound_effect("game_over")>game_over</Button>
        <Button on_click=move |_| play_sound_effect("hard_drop")>hard_drop</Button>
        <Button on_click=move |_| play_sound_effect("move")>move</Button>
        <Button on_click=move |_| play_sound_effect("soft_drop")>soft_drop</Button>
        <Button on_click=move |_| play_sound_effect("hold")>hold</Button>
        <Button on_click=move |_| play_sound_effect("pre_123")>pre_123</Button>
        <Button on_click=move |_| play_sound_effect("rotate")>rotate</Button>
    }
}