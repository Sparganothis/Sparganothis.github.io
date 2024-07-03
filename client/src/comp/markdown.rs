use leptos::*;

#[component]
pub fn Markdown(md_src: String) -> impl IntoView {
    let html_src = markdown::to_html(&md_src);
    view! {
        <div style="padding: 1vh; margin: 1vh;" inner_html=html_src></div>
    }
}