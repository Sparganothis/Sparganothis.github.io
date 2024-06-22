use std::string;

use game::tet::Tet;
use leptonic::input::TextInput;
use leptos::*;

#[component]
pub fn MultiSelectSmecher(
    
    #[prop(into)] options: MaybeSignal<Vec<Tet>>,
    #[prop(into)] selected: Signal<Vec<Tet>>,
    #[prop(into)] set_selected: Callback<Vec<Tet>>,)-> impl IntoView{

        // let analfabet = Tet::all().iter().map(|x|{
        //     format!("{:?}",*x)
        // }).collect::<Vec<_>>().as_slice().join("") ;
        let text = move || -> String{
            selected.get().into_iter().map(|c| {
                format!("{:?}", c)
            }).collect::<Vec<_>>().as_slice().join("")
        };
        let set_text = move |t: String|{
            let vt : Vec<Option<Tet>> = t.chars().map(|c| {
                match c {
                    'i'|'I' => {
                        Some(Tet::I)
                    },
                    'j'|'J' => {
                        Some(Tet::J)
                    },
                    's'|'S' => {
                        Some(Tet::S)
                    },
                    'z'|'Z' => {
                        Some(Tet::Z)
                    },
                    'o'|'O' => {
                        Some(Tet::O)
                    },
                    't'|'T' => {
                        Some(Tet::T)
                    },
                    'L'|'l' => {
                        Some(Tet::L)
                    },
                    
                    _=>None,

                }

            }).collect::<Vec<_>>();
            let vt = vt.iter().filter(|c| c.is_some()).map(|c| c.unwrap()).collect::<Vec<_>>();
            set_selected.call(vt);
        };
        let set_text = Callback::<String>::new(set_text);
    view!{
     {move || {format!("{:?}",options.get())}}
     <TextInput get=text.into_signal() set=set_text/>
    
    
    }
    }


fn penis() {
    struct vagin {
        pub puradel: u32,
    }

}