use crate::{comp::{game_board::RandomOpponentGameBoard, game_board_flex::FlexText, menu_grid_view::MenuGridView, table_match::AllMatchTable}, mobile_check::is_mobile_phone, websocket::demo_comp::call_api_sync_or_error};
use game::{api::{game_match::GameMatchType, websocket::{GetMatchListArg, StartMatch}}, tet::GameSeed};
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
    
   let play_button = view! {
       <div style="width:100%;height:100%; container-type: size;">
           <h3
               style="font-size:80cqh; text-align: center; cursor:pointer;"
               on:click=move |_| { obtain_new_match_id.call(()) }
           >
               PLAY
           </h3>
       </div>
       <Show
           when=move || (error_display.get().len() > 0)
           fallback=move || {
               view! {}
           }
       >

           <div style="width:100%;height:100%; container-type: size;">
               <p style="font-size:15cqh; text-align: center; color: red; margin:0px;">
                   {error_display.get()}
               </p>
           </div>
       </Show>

       <Show when=move || (waiting_for_game.get())>
           <div style="width:100%;height:100%; container-type: size;">
               <p style="font-size:15cqh; text-align: center; color: brown; margin:0px;">
                   Please Wait
               </p>
           </div>
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
   }.into_view();
   
   let play_button = if is_mobile_phone() {
    view! { <p style="color:red">You are phone. Plz use PC to play.</p> }.into_view()
} else {play_button};


    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            0 => {
                view! { <FlexText text="online"/> }.into_view()
            },
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            6=>play_button.clone(),
            _ => {                view!{                }.into_view()            },
        }
     }).collect();

    view! { <MenuGridView views/> }
}


#[component]
pub fn AllGamesMatchList() -> impl IntoView {
    view! { <AllMatchTable list_type=GetMatchListArg::BestGames/> }
}

