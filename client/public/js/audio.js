export function init_audio_js() {
    var files = {
        // "acccess_denied": "249300__suntemple__access-denied.wav.mp3",
        "game_over": ['effects/brass-144755.mp3', 0.5, false],
        // "poker_chip": "476818__victorium183__menuaccept.wav.mp3",
        // "swipe": "movement-swipe-whoosh-3-186577.mp3",
        "pre_123": ["effects/476818__victorium183__menuaccept.wav.mp3", 0.3, false],
        "soft_drop": ["effects/384187__malle99__click-tick.wav.mp3", 0.1, false]
        ,
        "move": ["effects/577020__nezuai__ui-sound-1.wav.mp3", 0.5, false],
        "rotate": ["effects/577020__nezuai__ui-sound-1.wav.mp3", 0.5, false],
        "hold": ["effects/switch-light-04-82204.mp3", 0.5, false],
        "hard_drop": ["effects/kick-183936.mp3", 0.5, false],
        "clear_line": ["effects/721159__nfrae__drunkard.wav.mp3", 0.5, false],

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

export function play_sound_js(sound_items, sound_name) {
    if (sound_name in sound_items) {

        var _play_id = sound_items[sound_name].play();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
    // sound_items[sound_name].volume(sound_volume, play_id);
}

export function stop_sound_js(sound_items, sound_name) {
    if (sound_name in sound_items) {

        var _play_id = sound_items[sound_name].stop();
    } else {
        console.error("SOUND DOES NOT EXIST!!!! : ----- >>> " + sound_name);
    }
    // sound_items[sound_name].volume(sound_volume, play_id);
}
