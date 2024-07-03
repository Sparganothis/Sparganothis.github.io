use leptos::*;

use crate::comp::markdown::Markdown;
#[component]
pub fn AboutPage() -> impl IntoView {
    view!{
        <div class="main_left">
            <Markdown md_src=include_str!("../../../.docs/lore.md").to_string()/>
        </div>
    }
}