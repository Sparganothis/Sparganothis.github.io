use leptos::*;

use crate::websocket::demo_comp::WebsocketAPI;
use crate::comp::game_board_flex::FlexText;

#[component]
pub fn WebsocketErrorDisplay() -> impl IntoView {
    let is_open_sig = create_rw_signal(false);
    let api2: WebsocketAPI = expect_context();
    let error_cnt = move || {
        api2.error_msgs.with(|x| x.len())
    };
    let overlay = move || {
        let is_open = is_open_sig.get();
        let errors = api2.error_msgs.get();
        let get_err =  move || errors.clone();
        if is_open {
            log::info!("overlay opened");
            view! {
                <div
                    style="position:absolute; left: 13vmin; top: 3vmin; height: 88vmin; width: 99vmin; border:1vmin solid red; z-index: 1999; background-color:#eee; cursor:pointer;"
                    on:click=move |_| { is_open_sig.set(!is_open_sig.get_untracked()) }
                >

                    "errors here lol"
                    <ul>
                        <For
                            each=get_err
                            key=|k| k.clone()
                            children=|k| {
                                view! { <li style="color:brown;">{k}</li> }
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

    let err_txt = (move || {
        format!("{} err", error_cnt())
    }).into_signal();

    view! {
        <div
            style="cursor:pointer;width:100%;height:100%;"
            on:click=move |_| { is_open_sig.set(!is_open_sig.get_untracked()) }
        >
            <FlexText text=err_txt size_cqh=60.0/>
        </div>
        {overlay}
    }
}