
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

        <Button on_click=move |_| play_sound_effect("pre_123_1")>pre_123_1</Button>
        <Button on_click=move |_| play_sound_effect("pre_123_2")>pre_123_2</Button>
        <Button on_click=move |_| play_sound_effect("pre_123_3")>pre_123_3</Button>
        <Button on_click=move |_| play_sound_effect("pre_123_4")>pre_123_4</Button>
        
        <Button on_click=move |_| play_sound_effect("clear_line_1")>clear_line_1</Button>
        <Button on_click=move |_| play_sound_effect("clear_line_2")>clear_line_2</Button>
        <Button on_click=move |_| play_sound_effect("clear_line_3")>clear_line_3</Button>
        <Button on_click=move |_| play_sound_effect("clear_line_4")>clear_line_4</Button>

        <Button on_click=move |_| play_sound_effect("game_over")>game_over</Button>
        <Button on_click=move |_| play_sound_effect("soft_drop")>soft_drop</Button>
        <Button on_click=move |_| play_sound_effect("hard_drop")>hard_drop</Button>
        <Button on_click=move |_| play_sound_effect("hold")>hold</Button>

        <Button on_click=move |_| play_sound_effect("move_left")>move_left</Button>
        <Button on_click=move |_| play_sound_effect("move_right")>move_right</Button>
        
        <Button on_click=move |_| play_sound_effect("rotate_left")>rotate_left</Button>
        <Button on_click=move |_| play_sound_effect("rotate_right")>rotate_right</Button>
    }
}