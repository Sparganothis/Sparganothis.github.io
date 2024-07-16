use leptos::*;
use leptonic::{checkbox::CheckboxSize, prelude::*};

use crate::comp::game_board_flex::FlexText;

#[component]
pub fn CheckboxRadioGroup(options: Vec<String>, value: RwSignal<String>) -> impl IntoView {
    if value.get_untracked().is_empty() {
        value.set(options.get(0).cloned().unwrap_or("???".to_string()));
    }
    let make_checkbox = move |name: String| {
        let n1 = name.clone();
        let n2 = name.clone();
        let n3 = name.clone();
        let n4 = name.clone();
        view! {
            <div style="display:flex; flex-direction:row;height:100%;width:25%;cursor:pointer;">
                <div style="width:20%; height:100%;align-content:center;">
                <Checkbox size=CheckboxSize::Big checked={move || n1 == value.get()}.into_signal() set_checked={let n2 = n2.clone();move |c:bool| value.set(n2.clone())} />
                </div>
                <div style="width:80%; height:100%" on:click=move |_| value.set(n4.clone())>
                    <FlexText text=n3.clone() size_cqh=60.0 />
                </div>
            </div>
        }
    };
    let checkboxes:Vec<_> = options.iter().map(|c| make_checkbox(c.clone())).collect();
    view!{
        <div style="display:flex; flex-direction:row;width:100%;height:100%">
            {checkboxes}
        </div>
    }
}

#[component]
pub fn FeedbackButtonExpander()->impl IntoView {
    let show_overlay = create_rw_signal(false);
    let on_click = move |_| {
        let old_val = show_overlay.get_untracked();
        let new_val = !old_val;
        show_overlay.set(new_val);
    };

    view!{
        <div style="position:absolute;right:1vmin;bottom:1vmin;width:14vmin;height:5vmin;z-index:2999;border:1vmin solid black; background-color:white;padding:0.1vmin;margin:0.1vmin;cursor: pointer;" on:click=on_click>
            <FlexText text="Feedback" size_cqh=60.0/>
        </div>
        <Show when=move || show_overlay.get()>
        
            <div style="position:absolute;left:1vw;bottom:10vh;width:84vw;height:80vh;z-index:3999;border:1vmin solid black; background-color:white;padding:0.1vmin;margin:0.1vmin;">

                <FeedbackOverlayForm/>
            </div>
        </Show>
    }
}

#[component]
pub fn FeedbackOverlayForm()->impl IntoView {
    let feedback_type = create_rw_signal("".to_string());
    let feedback_message = create_rw_signal("".to_string());
    let on_save = move |_| {

    };

    view!{
        <div style="height:10%;width:100%">
            <FlexText text="Feedback"/>
        </div>
        <div style="height:2%;width:100%">
        </div>
        <div style="height:4%;width:100%">
            <CheckboxRadioGroup options=vec!["Suggestion".to_string(), "Bug Report".to_string(), "Shitpost".to_string()] value=feedback_type/>
        </div>
        <div style="height:24%;width:100%">
            <textarea style="width:80%;height:80%;padding:5%;margin:5%;"
                on:input=move |ev| feedback_message.set(event_target_value(&ev))
            >
            </textarea>
        </div>

        <div style="height:15%;width:100%">
            <p>"MESSAGE:" {feedback_message}</p>
            <Button on_click=on_save color=ButtonColor::Info>
                "Send " {feedback_type}
            </Button>
        </div>
    }
}