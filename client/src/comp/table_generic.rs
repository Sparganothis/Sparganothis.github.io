use leptonic::button::ButtonColor;
use leptos::*;
use leptos_struct_table::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use game::api::table_paginate::TablePaginateDirection;

pub trait CustomRowExtraView {
    fn row_extra_view(&self) -> impl IntoView{
        view!{ }
    }
}




#[component]
pub fn DisplayTableGeneric<Item, KT>(
    fetch_items: Callback< (
        TablePaginateDirection<KT>,
        Callback<Vec<(KT, Item)>>
    )>, 
    column_display_fns:Vec<(String, Callback<(KT,Item), View>)>,

    #[prop(optional, into)]
    _item: PhantomData<Item>, 
    #[prop(optional, into)]
    _key_type: PhantomData<KT>,
) -> impl IntoView  
where 
    Item: Clone + Debug+'static,
    KT: Clone + Debug+'static,
{
    let items_signal: RwSignal<Vec<(KT, Item)>> = create_rw_signal(vec![]);

    fetch_items.call((
        TablePaginateDirection::<KT>::InitialPage,
        Callback::new(move |r| {
            items_signal.set(r);
    })));

    let table_from_rows = move || {
        let items = items_signal.get();
        
        let on_next = move |_| {
            let itemmms = items_signal.get_untracked();
            if itemmms.len() > 0 {
                let last = itemmms.last().unwrap();
                fetch_items.call(
                    (
                        TablePaginateDirection::<KT>::Forward(last.0.clone()), 
                        Callback::new(move |r| {
                            items_signal.set(r);
                })));
            }
        };
        let on_prev = move |_| {
            let fetched_items = items_signal.get_untracked();
            if fetched_items.len() > 0 {
                let first = fetched_items.first().unwrap();
                fetch_items.call((TablePaginateDirection::<KT>::Back(first.0.clone()), Callback::new(move |r| {
                    items_signal.set(r);
                })));
            }
        };
      
        let buttons = if items.len() > 0 {
            view! {
                <div style="display:flex; flex-direction:row;">

                    <leptonic::prelude::Button
                        style="margin-right:auto"
                        on_click=move |_| { on_prev(()) }
                        color=ButtonColor::Info
                    >
                        "PREV"
                    </leptonic::prelude::Button>

                    <leptonic::prelude::Button
                        on_click=move |_| { on_next(()) }
                        color=ButtonColor::Info
                    >
                        "NEXT"
                    </leptonic::prelude::Button>

                </div>
            }.into_view()
        } else {
            view!{}.into_view()
        };

        
        let table_headers:Vec<_> = column_display_fns.iter().map(|k| k.0.clone()) .map(|k| view!{<th>{k}</th>}) .collect();
     
        let table_rows: Vec<View> = items.iter().map(|r| {
            column_display_fns.iter().map(|(_c_name, c_fn)| {
                let val = c_fn.call(r.clone());

                view!{
                    <td>{val}</td>
                }.into_view()
            }).collect()
        }).map(|r:Vec<_>| view!{<tr>{r}</tr>}.into_view())
        .collect();
            
        view! {
            <div>
                <table>
                  <thead>
                    {table_headers}
                  </thead>
                  <tbody>
                    {table_rows}
                  </tbody>
                </table>
                {buttons}
            </div>
        }
        .into_view()
    };

    view! { {table_from_rows} }
}
