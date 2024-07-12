use std::str::FromStr;

use anyhow::Context;
use game::{api::{game_match::GameMatch, game_replay::GameId, websocket::{GetMatchInfo, GetSegmentCount, WhoAmI}}, bot::get_bot_from_id};
use leptos::*;
use leptos_router::use_params_map;

use crate::{comp::{game_board_bot::BotGameBoard, game_board_player::PlayerGameBoardFromId, game_board_spectator::SpectatorGameBoard}, websocket::demo_comp::call_api_sync};

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
    create_effect(move |_|{
        if let Ok(match_uuid) = url() {
            call_api_sync::<GetMatchInfo>(match_uuid, move |r:GameMatch| {
                match_info.set(Some((match_uuid, r)));
            });
        }
    });

    create_effect(move |_| {
        if let Some((_, match_info)) = match_info.get() {
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

            call_api_sync::<GetSegmentCount>(gameinfo_0, move |r| {
                ginfo_0.set(Some((gameinfo_0, r)));
            });


            call_api_sync::<GetSegmentCount>(gameinfo_1, move |r| {
                ginfo_1.set(Some((gameinfo_1, r)));
            });

        }
    });

    let guest_id = create_rw_signal(None);
    call_api_sync::<WhoAmI>((), move |r| {
        guest_id.set(Some(r));
    });

   let left_view = create_rw_signal(view!{}.into_view());
    let right_view = create_rw_signal(view!{}.into_view()); 

    let title_sig = create_rw_signal("".to_string());
    create_effect(move |_| {
        if let (
            Some(g0), 
            Some(g1), 
            Some(whoami), 
            Some(match_info)
        ) = (ginfo_0.get(), ginfo_1.get(), guest_id.get(), match_info.get()) {
            log::info!("===> got final effect");

            title_sig.set(match_info.1.title);
            let mut bot0 = None;
            let mut bot1 = None;

            let is_mine_0 = g0.0.user_id.eq(&whoami.user_id);
            let is_mine_1 = g1.0.user_id.eq(&whoami.user_id);

            if is_mine_0 {
                if let Ok(bot_name) = get_bot_from_id(g1.0.user_id) {
                    bot1 = Some(bot_name);
                }
            }
            if is_mine_1 {
                if let Ok(bot_name) = get_bot_from_id(g0.0.user_id) {
                    bot0 = Some(bot_name);
                }
            }

            let v0 = view! {
                <MatchGameBoard
                    game_id=g0.0
                    is_in_progress=g0.1.is_in_progress
                    is_mine=is_mine_0
                    is_my_bot=bot0
                />
            }.into_view();
             let v1 = view! {
                 <MatchGameBoard
                     game_id=g1.0
                     is_in_progress=g1.1.is_in_progress
                     is_mine=is_mine_1
                     is_my_bot=bot1
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
        <div class="main_left">{move || left_view.get()}</div>
        
        <div class="main_mid">
            <h1>vs.</h1>
            <p>{title_sig}</p>
        </div>
        
        <div class="main_right">{move || right_view.get()}</div>
    }
}


#[component]
pub fn MatchGameBoard(game_id: GameId, is_in_progress: bool, is_mine: bool, is_my_bot: Option<String>) -> impl IntoView {

    match (is_in_progress, is_mine, is_my_bot) {
        (false, _, _) => {
            view! { <SpectatorGameBoard game_id/> }.into_view()
        },
        (true, true, _) => {
            view! { <PlayerGameBoardFromId game_id=game_id/> }.into_view()
        },

        (true, false, Some(bot_name)) => {
            view! { <BotGameBoard game_id=game_id bot_name/> }.into_view()
        },

        (true, false, _) => {
            view! { <SpectatorGameBoard game_id/> }.into_view()
        }
    }
}
