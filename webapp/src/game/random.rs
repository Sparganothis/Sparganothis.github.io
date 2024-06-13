use super::tet::GameReplayEvent;
use super::tet::Tet;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
pub type GameSeed = <ChaCha20Rng as SeedableRng>::Seed;

fn get_rng(seed: &GameSeed) -> ChaCha20Rng {
    // let mut seed = seed.clone();
    // rand:    :thread_rng().fill(&mut seed);
    ChaCha20Rng::from_seed(*seed)
}

pub fn shuffle_tets(seed: &GameSeed, event_time: i64) -> (Vec<Tet>, GameSeed) {
    let event_time = event_time.to_le_bytes();
    let mut seed = *seed;
    for i in 0..8 {
        seed[i] ^= event_time[i];
    }

    let mut v = Tet::all();
    use rand::prelude::SliceRandom;
    let mut rng = get_rng(&seed);
    v.shuffle(&mut rng);
    let new_seed = rng.gen();
    (v, new_seed)
}
use std::hash::{DefaultHasher, Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn accept_event(
    seed: &GameSeed,
    event: &GameReplayEvent,
    event_ts: i64,
    event_idx: u32,
) -> GameSeed {
    let event_hash = calculate_hash(event).to_le_bytes();
    let ts = event_ts.to_le_bytes();
    let event_idx = event_idx.to_le_bytes();

    let mut rng = get_rng(seed);
    let more_bytes: [u8; 12] = rng.gen(); // 8 + 8 + 4 + 12 = 32

    let all_bytes: Vec<u8> = event_hash
        .iter()
        .chain(ts.iter())
        .chain(event_idx.iter())
        .chain(more_bytes.iter())
        .cloned()
        .collect();
    let all_bytes_len = all_bytes.len();

    let new_seed: GameSeed = match all_bytes.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!(
            "Expected a Vec of length {} but it was {}",
            32, all_bytes_len
        ),
    };

    new_seed
}
