use leptos::*;

use crate::websocket::demo_comp::WebsocketAPI;

#[component]
pub fn WebsocketErrorDisplay() -> impl IntoView {
    let is_open = create_rw_signal(false);
    let api2: WebsocketAPI = expect_context();
    let error_cnt = move || {
        api2.error_msgs.with(|x| x.len())
    };
    let overlay = move || {
        let is_open = is_open.get();
        let errors = api2.error_msgs.get();
        let get_err =  move || errors.clone();
        if is_open {
            log::info!("overlay opened");
            view! {
                <div style="position:absolute; left: 13vmin; top: 3vmin; height: 88vmin; width: 99vmin; border:1vmin solid red; z-index: 1999; background-color:#eee;">

                    "errors here lol" <ul>
                        <For
                            each=get_err
                            key=|k| k.clone()
                            children=|k| {
                                view! { <h1 style="color:brown;">{k}</h1> }
                            }
                        />

                    </ul>
                </div>
            }.into_view()
        } else {
            log::info!("overlay clozed");
            view! {

            }.into_view()
        }
    };

    view! {
        <h1 on:click=move |_| { is_open.set(!is_open.get_untracked()) }>{error_cnt}</h1>
        {overlay}
    }
}