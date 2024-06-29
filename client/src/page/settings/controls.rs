use game::api::user::{self, GuestInfo};
use leptonic::toggle::Toggle;
use leptos::*;

#[allow(unused_variables)]
#[component ]
pub fn ControlsSettingsTab(user_profile: user::UserProfile, guest_id: GuestInfo) -> impl IntoView
{
    
    let (state, set_state) = create_signal(false);

    view! {
        <table>
        <tr>
            <td>
                <Toggle state=state set_state=set_state/>
            </td>
            <td>
                <h3>I Have ADHD</h3>
            </td>
        </tr>
    </table>
    }
}