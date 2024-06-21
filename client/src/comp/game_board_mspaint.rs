use std::collections::VecDeque;

use game::tet::{CellValue, CurrentPcsInfo, GameState, Tet};
use leptonic::select::{Multiselect, Select};
use leptos::*;

use crate::comp::game_board::GameBoard;

#[component]
pub fn MsPaintPage()-> impl IntoView{
    let game_state = create_rw_signal(GameState::empty());
    view! {
        <div class="main_left">
            <MsPaintGameBoard game_state/>
        </div>
        <div class="main_right">
            <NextPeaceSelector game_state/>
            <CurrentPeaceSelector game_state/>
        </div>
    }
}

#[component]
pub fn CurrentPeaceSelector( game_state : RwSignal<GameState>)-> impl IntoView {
    let selected = move || game_state.with(|game_state| game_state.current_pcs.unwrap().tet);
    let set_selected = move |new_current:Tet|{
        game_state.update(|game_state|{
            let _ = game_state.main_board.delete_piece(&game_state.current_pcs.unwrap());
            game_state.current_pcs = Some(CurrentPcsInfo{ pos: new_current.spawn_pos(), tet: new_current, rs: game_state.current_pcs.unwrap().rs, id: game_state.current_pcs.unwrap().id });
            let _ = game_state.main_board.spawn_piece(&game_state.current_pcs.unwrap());
        });

    };
    view! {
        <h1>current pieces selector</h1>
        <Select
        options=Tet::all()
        search_text_provider=move |o| format!("{o:?}")
        render_option=move |o| format!("{o:?}")
        selected=selected
        set_selected=set_selected
    />
    }
}

#[component]
pub fn NextPeaceSelector( game_state : RwSignal<GameState>)-> impl IntoView {
let get_next = move || game_state.get().next_pcs.iter().cloned().collect::<Vec<_>>();
let set_next = move |v: Vec<Tet>| game_state.update(|game_state|{
    game_state.next_pcs=v.iter().cloned().collect::<VecDeque<_>>();
});    
    view! {
        <h1>next pieces selector</h1>
        <Multiselect
            options=Tet::all()
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
            selected=get_next
            set_selected=set_next
        />
    }
}
#[component]
pub fn MsPaintGameBoard( game_state : RwSignal<GameState>)-> impl IntoView {
    let on_reset_game = Callback::<()>::new(move |_| {
    });
    let on_click = Callback::<(i8,i8)>::new(move |(y, x)| {
        game_state.update(|game_state| {
            let old_value =  game_state.main_board.v[y as usize][x as usize];
            let new_value  =             match old_value {
                CellValue::Piece(_) => CellValue::Empty,
                CellValue::Garbage => CellValue::Empty,
                CellValue::Empty => CellValue::Piece(Tet::J),
                CellValue::Ghost =>  CellValue::Piece(Tet::J),
            };
            game_state.main_board.v[y as usize][x as usize] = new_value;
        })
    });
    
    view! {
        <h1>mspaint.exe</h1>
        <GameBoard game_state on_reset_game on_main_cell_click=on_click/>
    }
}
