use std::string;

use game::tet::Tet;
use leptonic::input::TextInput;
use leptos::*;

#[component]
pub fn MultiSelectSmecher(
    
    #[prop(into)] options: MaybeSignal<Vec<Tet>>,
    #[prop(into)] selected: Signal<Vec<Tet>>,
    #[prop(into)] set_selected: Callback<Vec<Tet>>,)-> impl IntoView{

        let analfabet = Tet::all().iter().map(|x|{
            format!("{:?}",*x)
        }).collect::<Vec<_>>().as_slice().join("") ;
        let text = || -> String{"Penis".to_string()};
        let set_text = |t: String|{
            log::info!("{}",t);
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