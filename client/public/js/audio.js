export function init_audio_js() {
    console.log("INIT AUDIO JS ");
    var files = {
        // "acccess_denied": "249300__suntemple__access-denied.wav.mp3",
        "game_over": ['effects/brass-144755.mp3', 0.9, false],
        // "poker_chip": "476818__victorium183__menuaccept.wav.mp3",
        // "swipe": "movement-swipe-whoosh-3-186577.mp3",
        "pre_123": ["effects/476818__victorium183__menuaccept.wav.mp3", 0.25, false],
        "soft_drop": ["effects/384187__malle99__click-tick.wav.mp3", 0.15, false]  ,
        "move": ["effects/577020__nezuai__ui-sound-1.wav.mp3", 0.5, false],
        "rotate": ["effects/movement-swipe-whoosh-3-186577-edit.mp3", 0.2, false],
        "hold": ["effects/switch-light-04-82204.mp3", 0.5, false],

        "hard_drop": ["effects/kick-183936.mp3", 0.20, false],
        "clear_line": ["effects/721159__nfrae__drunkard.wav.mp3", 0.25, false],

        "mmenu_mmusicc": ["music/Limosine.mp3", 0.5, true],

        // music
        // https://audionautix.com/free-music/acid-jazz
    };

    var result = {};

    for (var key in files) {
        var file_src = files[key][0];
        var volume = files[key][1];
        var should_retry_after_unlock =  files[key][1];
        console.log("registering key " + key + " and sound " + files[key]);

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

const ALL_INIT_AUDIO_STUFF = init_audio_js();

export function play_sound_js(sound_name, volume) {
    var old_volume = ALL_INIT_AUDIO_STUFF[sound_name].volume();

    if (sound_name in ALL_INIT_AUDIO_STUFF) {

    var _play_id = ALL_INIT_AUDIO_STUFF[sound_name].play();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
    ALL_INIT_AUDIO_STUFF[sound_name].volume(old_volume * volume* 0.01, _play_id);
}

export function stop_sound_js(sound_name) {
    if (sound_name in ALL_INIT_AUDIO_STUFF) {

    var _play_id = ALL_INIT_AUDIO_STUFF[sound_name].stop();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
}
