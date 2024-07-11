export function init_audio_js() {
    console.log("INIT AUDIO JS ");
    var files = {

        "pre_123_1": ["effects/80s_synth/pre123_1_menu_C_short_reverb.mp3", 1.0, false],
        "pre_123_2": ["effects/80s_synth/pre123_2_menu_x_back.wav.mp3", 1.0, false],
        "pre_123_3": ["effects/80s_synth/pre123_3_menu_x_fwd.wav.mp3", 1.0, false],
        "pre_123_4": ["effects/80s_synth/pre123_4_Asharp_clear.wav.mp3", 1.0, false],

        "clear_line_1": ["effects/80s_synth/menu_C_silent.wav.mp3", 1.0, false],
        "clear_line_2": ["effects/80s_synth/menu_C_short_muddy.mp3", 0.95, false],
        "clear_line_3": ["effects/80s_synth/menu_C_clear.wav.mp3", 0.60, false],
        "clear_line_4": ["effects/80s_synth/menu_C_short_wah.mp3", 0.65, false],
        
        "game_over": ['effects/jazz_drums/JAZZ_DRUMS_1_CRASH_CYMBAL.wav.mp3', 1.0, false],
        "soft_drop": ["effects/jazz_drums/JAZZ_DRUMS_1_CYMBAL_2.wav.mp3", 0.4, false]  ,
        "hard_drop": ["effects/jazz_drums/JAZZ_DRUMS_1_KICK.wav.mp3", 1.0, false],
        "hold": ["effects/jazz_drums/JAZZ_DRUMS_1_BELL.wav.mp3", 1.0, false],

        "move_left": ["effects/jazz_drums/JAZZ_DRUMS_1_HI_HAT_SHORT.wav.mp3", 0.7, false],
        "move_right": ["effects/jazz_drums/JAZZ_DRUMS_1_HI_HAT_END_2.wav.mp3", 0.7, false],

        "rotate_left": ["effects/jazz_drums/JAZZ_DRUMS_1_SNARE_1.wav.mp3", 0.6, false],
        "rotate_right": ["effects/jazz_drums/JAZZ_DRUMS_1_SNARE_2.wav.mp3", 0.6, false],

        "mmenu_mmusicc": ["music/Limosine.mp3", 0.5, true],

        // music
        // https://audionautix.com/free-music/acid-jazz
    };

    var result = {};

    for (var key in files) {
        var file_src = files[key][0];
        var volume = files[key][1];
        var should_retry_after_unlock =  files[key][1];
        // console.log("registering key " + key + " and sound " + files[key]);

        var arg = {
            src: ["/public/audio/"+file_src],
            volume: volume,
          };
        if (should_retry_after_unlock) {
            // for background music
            arg["onplayerror"] = function() {
              sound.once('unlock', function() {
                sound.play();
              });
            };
        }
        var sound = new Howl(arg);
        result[key] = sound;
    }
    return result;
}

const MUSIC_PLAY_ID = {};
const ALL_INIT_AUDIO_STUFF = init_audio_js();

export function play_sound_js(sound_name, volume) {
    var old_volume = ALL_INIT_AUDIO_STUFF[sound_name].volume();

    if (sound_name in ALL_INIT_AUDIO_STUFF) {

    var _play_id = ALL_INIT_AUDIO_STUFF[sound_name].play();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
    ALL_INIT_AUDIO_STUFF[sound_name].volume(old_volume * volume* 0.01, _play_id);
    MUSIC_PLAY_ID[sound_name]=_play_id;
   // return _play_id;
}

export function stop_sound_js(sound_name) {
    if (sound_name in ALL_INIT_AUDIO_STUFF) {

    var _play_id = ALL_INIT_AUDIO_STUFF[sound_name].stop();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
}

export function change_global_volume_js(volume){
    Howler.volume(volume*0.01);   
}

export function change_sound_volume_js(sound_name, volume){
    if ((sound_name in MUSIC_PLAY_ID) && (sound_name in ALL_INIT_AUDIO_STUFF)){
        ALL_INIT_AUDIO_STUFF[sound_name].volume(volume* 0.01, MUSIC_PLAY_ID[sound_name]);
    }   
}

export function stop_all_sound_js(){
    Howler.stop(); 
}