use game::api::user::{self, GuestInfo};
use leptonic::{color::HSV, color_picker::ColorPicker};
use leptos::*;


#[allow(unused_variables)]
#[component]
pub fn ThemeSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {
    
    let (hsv, set_hsv) = create_signal(HSV::new());

    view!{
        <div style="width:50%; height: 20%">
        <ColorPicker hsv=hsv set_hsv=set_hsv/>
        </div>

    }
}


