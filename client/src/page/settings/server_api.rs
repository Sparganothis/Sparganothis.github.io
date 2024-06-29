use game::api::user_settings::{ControlSettingType, SoundSettingType, UserSettingType};
use game::api::websocket::{GetUserSetting, SetUserSetting};
use leptos::*;
use leptos::{create_rw_signal, RwSignal};
use serde::{Deserialize, Serialize};

use crate::websocket::demo_comp::call_api_sync_or_error;


#[derive(
    Copy, Debug, Clone,
)]
pub struct UserSettingSignals{
    pub sound_enabled: RwSignal<bool>,
    pub sound_menu_music_enabled: RwSignal<bool>,
    pub sound_all_sounds_volume: RwSignal<f64>,
    pub sound_menu_music_volume: RwSignal<f64>,

    pub control_i_have_adhd : RwSignal<bool>,
} 

fn generic_usersetting_websocket_reactor<T>
( _type: UserSettingType,default_value: T,) 
-> RwSignal<T>
where T: Serialize + for<'a> Deserialize<'a> + Clone + std::fmt::Debug + PartialEq
{
    let signal= create_rw_signal(default_value);
    let is_from_server = create_rw_signal(false);
    
    log::info!("Fetching Setting vluae for {:?} ...", _type);
    call_api_sync_or_error::<GetUserSetting>(
        _type, 
        move |_setting_bytes|        {
            log::info!("OK: Fetched Setting vluae for {:?} len {} bytes", _type, _setting_bytes.len());
            if _setting_bytes.len() > 0 {
                let new_value = bincode::deserialize(&_setting_bytes).unwrap();
                is_from_server.set_untracked(true);
                signal.set(new_value);
            } else {
                // we hgave no value from server
            }
    }, move |err| {
        log::error!("update setting failed{:?}",err);
    });

    let _ = watch(
        move || signal.get(),
        move |updated_val, _old_val, _| {
            let is_changed = _old_val.is_none() || *_old_val.unwrap() != *updated_val;
            log::info!("setting watch change: _type={_type:?},  is_changed={is_changed:?} updated={updated_val:?}, old={_old_val:?} is_from_server={:?}", is_from_server.get_untracked());

            if is_from_server.get_untracked() {
                is_from_server.set_untracked(false);
                return;
            }

            if !is_changed {
                return;
            }
            let bytes = bincode::serialize(updated_val).unwrap();
            call_api_sync_or_error::<SetUserSetting>((_type,bytes),move |_:()|{

                log::info!("update setting ok type{:?}",_type);
            }, move |err|{
                log::error!("update setting failed{:?}",err);
            });

        },
        false,
    );

    signal
    
}

pub fn provide_user_setting(){
    let sound_disabled =
            generic_usersetting_websocket_reactor(
        UserSettingType::SoundSetting(
                SoundSettingType::EnableAllSounds
            ),
            false, 
        );

    let sound_menu_music_disabled =     generic_usersetting_websocket_reactor(
        UserSettingType::SoundSetting(
                SoundSettingType::EnableMenuMusic
            ),
            false, 
        );
 
    let sound_all_sounds_volume= generic_usersetting_websocket_reactor(UserSettingType::SoundSetting(SoundSettingType::AllSoundsVolume), 50.0);

    let sound_menu_music_volume= generic_usersetting_websocket_reactor(UserSettingType::SoundSetting(SoundSettingType::MenuMusicVolume), 50.0);

    let control_i_have_adhd = generic_usersetting_websocket_reactor(
        UserSettingType::ControlSetting(
                ControlSettingType::IHaveADHD
            ),
            false, 
        );

    leptos::provide_context(UserSettingSignals{
        sound_enabled: sound_disabled,
        sound_all_sounds_volume,
        sound_menu_music_volume,
        sound_menu_music_enabled: sound_menu_music_disabled,
        control_i_have_adhd,
    });
}

