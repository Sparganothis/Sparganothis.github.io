use game::{api::{game_match::GameMatchType, game_replay::GameId}, tet::{GameOverReason, GameState}, timestamp::get_timestamp_now_nano};
use leptos::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions};
use crate::comp::{game_board_flex::FlexText, game_board_player::PlayerGameBoardFromId};

#[component]
pub fn Game1PPage() -> impl IntoView {
    let params = use_params_map();
    let play_comp = move || {

        let game_id_str = params.with(|params| (params.get("game_id").cloned().unwrap_or("".to_string())));
        let game_type_str = params.with(|params| (params.get("game_type").cloned().unwrap_or("".to_string())));
        if let( Ok(gid), Ok(game_type)) = (GameId::from_url(game_id_str), GameMatchType::from_url(&game_type_str)) {

            view!{
                <PlayGame1POfType match_type=game_type game_id=gid />
            }.into_view()
        } else {
            view!{<FlexText text="bad url???"/>}.into_view()
        }
    }.into_view();

    view!{
        <div class="main_left">
            {play_comp}
        </div>
    }
}

#[component]
pub fn PlayGame1POfType(game_id: GameId, match_type: Option<GameMatchType>) -> impl IntoView {

    let text = GameMatchType::to_url(&match_type);
    let lobby_url = format!("/lobby/{}", text);
    let on_reset = Callback::new(move |_|{
        let navigate = use_navigate();             
        navigate(&lobby_url, NavigateOptions::default());
    });

    let control_callback = Callback::<_,_>::new(move |mut s:GameState | {
        match match_type {
            None => (),
            Some(GameMatchType::_40lines) => {
                if s.total_lines >= 40 {
                    s.game_over_reason = Some(GameOverReason::Win);
                }
            },
            
            Some(GameMatchType::blitz) => {
                let num_seconds = 120;
                if (get_timestamp_now_nano() - s.start_time) > num_seconds * 1000000 {
                    s.game_over_reason = Some(GameOverReason::Win);
                }
            }
            _ => {},
        }
        s
    } );

    view! {
            <PlayerGameBoardFromId game_id=game_id on_reset control_callback /> 
    }
}
