use leptos::*;
use leptonic::prelude::*;

pub fn you_are_phone_view() -> View {
    view! {
        <div style="container-type:size;  display: flex; flex-direction: column;
        justify-content: center;
        align-items: center;
        text-align: center;
        min-height: 100vh;">
            <h1 style="font-size:9cqmin; margin: 1cqmin; padding: 1cqmin;">
                You are phone.
            </h1>
            <p style="font-size:6cqmin;margin: 1cqmin; padding: 1cqmin; width: 50%;">
                Please use Firefox on PC or maybe the other thing.
            </p>
            <p style="font-size:6cqmin;margin: 1cqmin; padding: 1cqmin;">
                <a
                    target="_blank"
                    href="https://github.com/Sparganothis/Sparganothis.github.io"
                >
                    <Icon icon=icondata::BsGithub width="3vmin" height="3vmin"/>

                    "github.com/Sparganothis"
                </a>
            </p>
        </div>
    }.into_view()
}