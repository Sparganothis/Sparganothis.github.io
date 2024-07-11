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
        log::warn!("found {}
         rows", items.len());
        
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




#[component]
pub fn DisplayTableGeneric_OLD<Item, ItemRow,KT, DataP>(
    fetch_items: Callback< (
        TablePaginateDirection<KT>,
        Callback<Vec<(KT, Item)>>
    )>, 
    #[prop(optional, into)]
    _item: PhantomData<Item>, 
    #[prop(optional, into)]
    _row: PhantomData<ItemRow>,
    #[prop(optional, into)]
    _key_type: PhantomData<KT>,
    // #[prop(optional, into)]
    // _cls_p: PhantomData<ClsP>,
    #[prop(optional, into)]
    _data_p: PhantomData<DataP>,

) -> impl IntoView  
where 
    ItemRow: TableRow + Clone + Debug + From<(KT, Item)>+'static + CustomRowExtraView,
    Item: Clone + Debug+'static,
    KT: Clone + Debug+'static,
    // ClsP: Clone + Debug + 'static,
    DataP: TableDataProvider<ItemRow> + 'static + std::convert::From<std::vec::Vec<ItemRow>>
{
    // 
    let items_signal: RwSignal<Vec<(KT, Item)>> = create_rw_signal(vec![]);

    
    log::warn!("do the needful");

    // get from server 
    // call_api_sync::<GetMatchList>(list_type, move |_r| {
    //     all_games.set(_r);
    // });

    fetch_items.call((
        TablePaginateDirection::<KT>::InitialPage,
        Callback::new(move |r| {
            log::warn!("fromm mcallaback !!!");
            items_signal.set(r);
    })));

    let table_from_rows = move || {
        let items = items_signal.get();
        log::warn!("found {}
         rows", items.len());
        let rows = items
            .iter()
                .map(|r| ItemRow::from(r.clone()))
                .collect::<Vec<_>>();

            // leptos::html::table().child(
            //     TableContent(TableContentProps { rows: rows.into(), ..Default::default() })
            // ).into_view()

        
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

        
        #[allow(unused_variables, non_snake_case)]
        pub fn CustomTableRowRenderer<ItemRow>
        (
            // The class attribute for the row element. Generated by the classes provider.
            class: Signal<String>,
            // The row to render.
            row: ItemRow,
            // The index of the row. Starts at 0 for the first body row.
            index: usize,
            // The selected state of the row. True, when the row is selected.
            selected: Signal<bool>,
            // Event handler callback when this row is selected
            on_select: EventHandler<web_sys::MouseEvent>,
            // Event handler callback for changes
            on_change: EventHandler<ChangeEvent<ItemRow>>,
        ) -> impl IntoView 
        where ItemRow: Clone + TableRow + CustomRowExtraView {
            let row2 = row.clone();
            let row3 = row.clone();
            view! {
                <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
                    {row2.render_row(index, on_change)}
                    <td>{row3.row_extra_view()}</td>
                </tr>
            }
        }

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
            
        view! {
            <div>
                <table>
                    <TableContent<ItemRow, DataP, String, _> rows=rows.into() row_renderer=CustomTableRowRenderer />
                </table>
                {buttons}
                
            </div>
        }
        .into_view()
    };

    view! { {table_from_rows} }
}