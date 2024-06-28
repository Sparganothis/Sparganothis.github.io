use crate::{comp::table_match::AllMatchTable, websocket::demo_comp::call_api_sync_or_error};
use game::api::{game_match::GameMatchType, websocket::{GetMatchListArg, StartMatch}};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
#[component]
pub fn Game2LobbyPage() -> impl IntoView {
    view! {
        <div class="main_left">
            <Lobby2P/>
        </div>
        <div class="main_right">
            <AllGamesMatchList/>
        </div>
    }
}

#[component]
pub fn Lobby2P() -> impl IntoView {
    use leptonic::prelude::*;
    let match_id_signal = create_rw_signal(None);
    let waiting_for_game = create_rw_signal(false);
    let error_display = create_rw_signal("".to_string());

    let obtain_new_match_id: Callback<()> = Callback::new(move |_| {
        waiting_for_game.set(true);
        log::info!("waiting for game...");

        call_api_sync_or_error::<StartMatch>(GameMatchType::_1v1, move |r| {
            waiting_for_game.set(false);
            match_id_signal.set(Some(r));
        }, move |err_str| {
            waiting_for_game.set(false);
            error_display.set(err_str);
        });

    });

    create_effect(move |_| {
        if let Some(newgame) = match_id_signal.get() {
            let match_id = newgame.0;
            let url = format!("/match/{}", match_id);
            let navigate = use_navigate();
            navigate(&url, NavigateOptions::default());
        }
    });

    view! {
        <Show
            when=move || waiting_for_game.get()
            fallback=move || {
                view! {}
            }
        >

            <h1>WAITING FOR GAME</h1>
        </Show>

        <Show
            when=move || (error_display.get().len() > 0)
            fallback=move || {
                view! {}
            }
        >

            <h1 style="color:red">{error_display}</h1>
        </Show>

        <Show
            when=move || { !waiting_for_game.get() && match_id_signal.get().is_none() }
            fallback=move || {
                view! {}
            }
        >

            <Button
                on_click=move |_| obtain_new_match_id.call(())
                color=ButtonColor::Primary
            >
                "PLAY"
            </Button>

        </Show>

        <h1>
            {move || {
                match_id_signal
                    .with(|s| {
                        match s {
                            Some(x) => format!("{x:?}"),
                            None => "".to_string(),
                        }
                    })
            }}

        </h1>
    }
}



#[component]
pub fn AllGamesMatchList() -> impl IntoView {
    view! { <AllMatchTable list_type=GetMatchListArg::BestGames/> }
}