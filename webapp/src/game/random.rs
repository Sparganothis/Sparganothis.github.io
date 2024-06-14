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

pub fn accept_event(
    seed: &GameSeed,
    event: &GameReplayEvent,
    event_ts: i64,
    event_idx: u32,
) -> GameSeed {
    let event_hash = bincode::serialize(event).unwrap(); // 5 bytes
    assert!(event_hash.len() == 5);
    let ts = event_ts.to_le_bytes();
    let event_idx = event_idx.to_le_bytes();

    let mut rng = get_rng(seed);
    let more_bytes: [u8; 15] = rng.gen(); // 5 + 8 + 4 + 15 = 32

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
    let mut new_gen = get_rng(&new_seed);

    new_gen.gen()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    pub fn random_have_pinned_results() {
        let encoded_str1 = bincode::serialize(&crate::game::tet::TetAction::SoftDrop).unwrap();
        let encoded_str2 = bincode::serialize(&crate::game::tet::TetAction::MoveLeft).unwrap();
        let expected_str1: Vec<u8> = vec![1, 0, 0, 0];
        let expected_str2: Vec<u8> = vec![2, 0, 0, 0];
        assert_eq!(encoded_str1, expected_str1);
        assert_eq!(encoded_str2, expected_str2);

        let evt1 = GameReplayEvent {
            action: crate::game::tet::TetAction::SoftDrop,
            game_over: false,
        };

        let encoded_evt1 = bincode::serialize(&evt1).unwrap();
        let expected_str3: Vec<u8> = vec![1, 0, 0, 0, 0];
        assert_eq!(encoded_evt1, expected_str3);

        let seed = [0; 32];
        let event = GameReplayEvent {
            action: crate::game::tet::TetAction::MoveLeft,
            game_over: true,
        };
        let result = accept_event(&seed, &event, 0, 0);
        let expected_result = [
            51, 52, 212, 146, 170, 34, 166, 63, 51, 161, 145, 215, 91, 5, 109, 45, 185, 19, 32, 55,
            249, 73, 198, 204, 53, 200, 210, 77, 185, 40, 88, 62,
        ];
        assert_eq!(result, expected_result);
    }
}