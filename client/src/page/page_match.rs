use std::str::FromStr;

use anyhow::Context;
use game::api::{game_replay::GameId, websocket::{GetMatchInfo, GetSegmentCount, WhoAmI}};
use leptos::*;
use leptos_router::use_params_map;

use crate::{comp::{game_board_player::PlayerGammeBoardFromId, game_board_spectator::SpectatorGameBoard}, websocket::demo_comp::{call_websocket_api, WebsocketAPI}};

#[component]
pub fn MatchPage() -> impl IntoView {
    
    let params = use_params_map();
    let url = move || -> anyhow::Result<uuid::Uuid> {
        let x = params.with(|params| params.get("match_id").cloned());
        let x = x.context("no uuid given for matcch_id")?;
        let x = uuid::Uuid::from_str(&x)?;
        Ok(x)
    };

    let ginfo_0 = create_rw_signal(None);
    let ginfo_1  = create_rw_signal(None);
    
    let match_info = create_rw_signal(None); 
    let api = expect_context::<WebsocketAPI>();
    create_effect(move |_|{
        let api = api.clone();
        if let Ok(match_uuid) = url() {
            spawn_local(async move {
                
                let api2 = api.clone();
                let api = api.clone();
                if let Ok(fut)=call_websocket_api::<GetMatchInfo>(api2, match_uuid)
                {
                    let r = fut.await;
                    if let Ok(r) = r {
                        match_info.set(Some((match_uuid, r.clone())));
                        log::info!("====> got match info");

                        let match_info=r;
                        let gameinfo_0 = GameId {
                            user_id: match_info.users[0],
                            init_seed: match_info.seed,
                            start_time: match_info.time,
                        };
                        let gameinfo_1 = GameId {
                            user_id: match_info.users[1],
                            init_seed: match_info.seed,
                            start_time: match_info.time,
                        };
            
                        let api2 = api.clone();
                        spawn_local(async move {
                            if let Ok(fut)=call_websocket_api::<GetSegmentCount>(api2, gameinfo_0)
                            {
                                let r = fut.await;
                                log::info!("{r:?}");
                                if let Ok(r) = r {
                                    ginfo_0.set(Some((gameinfo_0, r)));
                                    log::info!("===> set ginfo_0 ===>");
                                }
                            }
                        });
            
                        let api2 = api.clone();
                        spawn_local(async move {
                            if let Ok(fut)=call_websocket_api::<GetSegmentCount>(api2, gameinfo_1)
                            {
                                let r = fut.await;
                                log::info!("{r:?}");
                                if let Ok(r) = r {
                                    ginfo_1.set(Some((gameinfo_1, r)));
                                    log::info!("===> set ginfo_1");
                                }
                            }
                        });

                    }
                }
            });
        }
    });


    let api: WebsocketAPI = expect_context();
    #[allow(unused_variables)]
    let guest_id = create_resource(
        || (),
        move |_| {
            let api_bis = api.clone();
            async move {
                // log::info!("calling websocket api");
                let r = call_websocket_api::<WhoAmI>(api_bis, ())
                    .expect("cannot obtain future")
                    .await;
                log::info!("===> whoami OK");
                r
            }
        },
    );


   let left_view = create_rw_signal(view!{}.into_view());
    let right_view = create_rw_signal(view!{}.into_view()); 

    let title_sig = create_rw_signal("".to_string());
    create_effect(move |_| {
        if let (
            Some(g0), 
            Some(g1), 
            Some(Ok(whoami)), 
            Some(match_info)
        ) = (ginfo_0.get(), ginfo_1.get(), guest_id.get(), match_info.get()) {
            log::info!("===> got final effect");

            title_sig.set(match_info.1.title);
            let v0 = view! {
                <MatchGameBoard
                    game_id=g0.0
                    is_in_progress=g0.1.is_in_progress
                    is_mine=g0.0.user_id.eq(&whoami.user_id)
                />
            }.into_view();
             let v1 = view! {
                 <MatchGameBoard
                     game_id=g1.0
                     is_in_progress=g1.1.is_in_progress
                     is_mine=g1.0.user_id.eq(&whoami.user_id)
                 />
             }.into_view();

            if g1.0.user_id.eq(&whoami.user_id) {
                left_view.set(v1);
                right_view.set(v0);
            } else {
                left_view.set(v0);
                right_view.set(v1);
            }
        };
    });
    
    view! {
        <h1>{title_sig}</h1>
        <div class="main_left">{move || left_view.get()}</div>
        <div class="main_right">{move || right_view.get()}</div>
    }
}


#[component]
pub fn MatchGameBoard(game_id: GameId, is_in_progress: bool, is_mine: bool) -> impl IntoView {

    match (is_in_progress, is_mine) {
        (false, _) => {
            view! { <SpectatorGameBoard game_id/> }.into_view()
        },
        (true, true) => {
            view! { <PlayerGammeBoardFromId new_game_id=game_id/> }.into_view()
        },

        (true, false) => {
            view! { <SpectatorGameBoard game_id/> }.into_view()
        }
    }
}
