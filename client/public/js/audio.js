export function init_audio_js() {
    var files = {
        // "acccess_denied": "249300__suntemple__access-denied.wav.mp3",
        "game_over": ['effects/brass-144755.mp3', 0.5],
        // "click": "",
        // "poker_chip": "476818__victorium183__menuaccept.wav.mp3",
        // "dunk": "721159__nfrae__drunkard.wav.mp3",
        // "swipe": "movement-swipe-whoosh-3-186577.mp3",
        "pre_123": ["effects/476818__victorium183__menuaccept.wav.mp3", 0.3],
        "soft_drop": ["effects/384187__malle99__click-tick.wav.mp3", 0.1]
        ,
        "move": ["effects/577020__nezuai__ui-sound-1.wav.mp3", 0.5],
        "rotate": ["effects/577020__nezuai__ui-sound-1.wav.mp3", 0.5],
        "hold": ["effects/switch-light-04-82204.mp3", 0.5],
        "hard_drop": ["effects/kick-183936.mp3", 0.5],

        // music
        // https://audionautix.com/free-music/acid-jazz
    };

    var result = {};

    for (var key in files) {
        var file_src = files[key][0];
        var volume = files[key][1];
        console.log("registering key " + key + " and sound " + files[key]);

        var sound = new Howl({
            src: ["/public/audio/"+file_src],
            volume: volume,
            // for background music
            // onplayerror: function() {
            //   sound.once('unlock', function() {
            //     sound.play();
            //   });
            // }
          });
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