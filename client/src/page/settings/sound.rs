use game::api::user::{self, GuestInfo};
use leptonic::{slider::Slider, toggle::Toggle};
use leptos::*;


#[allow(unused_variables)]
#[component]
pub fn SoundSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView {
    
    let (state, set_state) = create_signal(false);
    let (value_menu_mmusic , set_value_menu_music) = create_signal(0.0);
    view!{
        <table>
        <tr>
            <td>
                <Toggle state=state set_state=set_state/>
            </td>
            <td>
                <h3>Menu Music</h3>
            </td>
            <td>
                <h3>
                    {move || {
                        (if state.get() { "ON" } else { "OFF" }).to_string()
                    }}

                </h3>
            </td>
        </tr>
        <tr>
            <td style="width:20vmin;">
                <Slider
                    min=0.0
                    max=100.0
                    step=1.0
                    value=value_menu_mmusic
                    set_value=set_value_menu_music
                    value_display=move |v| format!("{v:.0}")
                />
            </td>
            <td>
                <h3>Menu Music Volume</h3>
            </td>
            <td>{move || format!("{}", value_menu_mmusic.get())}</td>
        </tr>
        <tr>
            <td>
                <Toggle state=state set_state=set_state/>
            </td>
            <td>
                <h3>Menu Music</h3>
            </td>
        </tr>
    </table>
    }
}


