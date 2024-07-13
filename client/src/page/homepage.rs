
// use game::tet::GameState;
use leptos::*;


use crate::
    comp::{game_board::RandomOpponentGameBoard, game_board_flex::FlexText, menu_grid_view::MenuGridView}
;

#[component]
pub fn Homepage()-> impl IntoView{
    let seed = [0;32];
    let views:Vec<_> = {0..20}.into_iter().map(|x|{
        match x{
            0 => {
                view! { <FlexText text="todo"/> }.into_view()
            },
            8 =>view! { <RandomOpponentGameBoard seed=seed/> }
            .into_view(),
            _ => {
                view!{

                }.into_view()
            }
        }
     }).collect();

    view! { <MenuGridView views/> }
}
