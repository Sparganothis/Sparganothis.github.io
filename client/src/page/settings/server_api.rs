use game::api::user_settings::{SoundSettingType, UserSettingType};
use game::api::websocket::SetUserSetting;
use leptos::*;
use leptos::{create_rw_signal, RwSignal};

use crate::websocket::demo_comp::{_call_websocket_api, call_api_sync, call_api_sync_or_error};


#[derive(
    Copy, Debug, Clone,
)]
pub struct UserSettingSignals{
    pub sound_disabled: RwSignal<bool>,
    pub sound_volume: RwSignal<f64>,
} 

pub fn provide_user_setting(){
    let sound_disabled= create_rw_signal(false);
    let _ = watch(
        move || sound_disabled.get(),
        move |disabled, _, _| {
            let bytes = bincode::serialize(disabled).unwrap();
            let _type = UserSettingType::SoundSetting(SoundSettingType::DisableAllSounds);
            call_api_sync_or_error::<SetUserSetting>((_type,bytes),move |_:()|{

                log::info!("update setting ok type{:?}",_type);
            }, move |err|{
                log::error!("update setting failed{:?}",err);
            })
        },
        false,
    );
    let sound_volume= create_rw_signal(50.0);

    leptos::provide_context(UserSettingSignals{
        sound_disabled,
        sound_volume,
    });
}

