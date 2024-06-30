use crate::{comp::{game_board::RandomOpponentGameBoard, menu_grid_view::MenuGridView, table_match::AllMatchTable}, websocket::demo_comp::call_api_sync_or_error};
use game::{api::{game_match::GameMatchType, websocket::{GetMatchListArg, StartMatch}}, random::GameSeed};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
#[component]
pub fn Game2LobbyPage() -> impl IntoView {
    view! {
      <Lobby2P/>
        <div class="main_right">
            <AllGamesMatchList/>
        </div>
    }
}

#[component]
pub fn Lobby2P() -> impl IntoView {
    let seed: GameSeed = [0; 32];
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

   // let redirect_to_new_game:todo
    

    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            8 =>view! { 
                <RandomOpponentGameBoard 
                seed=seed/> 
            }
            .into_view(),
            7=>view! {
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
            
                     <h1 on:click=move |_| { obtain_new_match_id.call(()) }>PLAY</h1>
            
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
                }.into_view(),
            _ => {                view!{                }.into_view()            }
        }
     }).collect();

    view! { <MenuGridView views/> }
}


#[component]
pub fn AllGamesMatchList() -> impl IntoView {
    view! { <AllMatchTable list_type=GetMatchListArg::BestGames/> }
}

