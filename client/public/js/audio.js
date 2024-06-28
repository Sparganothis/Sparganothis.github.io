export function init_audio_js() {
    var files = {
        "acccess_denied": "249300__suntemple__access-denied.wav.mp3",
        "game_over": 'brass-144755.mp3',
        "click": "384187__malle99__click-tick.wav.mp3",
        "poker_chip": "476818__victorium183__menuaccept.wav.mp3",
        "dunk": "721159__nfrae__drunkard.wav.mp3",
        "swipe": "movement-swipe-whoosh-3-186577.mp3",
        "ui_sound": "577020__nezuai__ui-sound-1.wav.mp3",
        "switch": "switch-light-04-82204.mp3"
    };

    var result = {};

    for (var key in files) {
        console.log("registering key " + key + " and sound " + files[key]);
        result[key] =  new Howl({
            src: ["/public/audio/"+files[key]],
            volume: 0.5,
        });
    }
    return result;
}

export function play_sound_js(sound_items, sound_name) {
    sound_items[sound_name].play();
}