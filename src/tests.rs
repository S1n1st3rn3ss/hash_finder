#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::*;

    // check basic algorithm correctness
    #[test]
    fn hash_digest_correctness() {
        assert_eq!(create_hash_digest(1), "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b");
        assert_eq!(create_hash_digest(2), "d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35");
        assert_eq!(create_hash_digest(3), "4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce");
    }
    #[test]
    fn example_one() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || find_relevant_hashes(3, 6, tx));
        let mut list: HashMap<i32, String> = Default::default();
        for received in rx {
            list.insert(received.clone().unwrap().0, received.clone().unwrap().1);
        }
        assert_eq!(list[&4163], "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000");
        assert_eq!(list[&11848], "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000");
        assert_eq!(list[&12843], "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000");
        assert_eq!(list[&13467], "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000");
        assert_eq!(list[&20215], "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000");
        assert_eq!(list[&28892], "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000");
    }
    #[test]
    fn example_two() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || find_relevant_hashes(5, 3, tx));
        let mut list: HashMap<i32, String> = Default::default();
        for received in rx {
            list.insert(received.clone().unwrap().0, received.clone().unwrap().1);
        }
        assert_eq!(list[&828028], "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000");
        assert_eq!(list[&2513638], "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000");
        assert_eq!(list[&3063274], "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000");
    }
    #[test]
    fn not_enough_hashes() {
        let (tx, rx) = mpsc::channel();
        // unrealistic values to not use actually impossible ones
        thread::spawn(move || find_relevant_hashes(16, 16, tx));

        for received in rx {
            assert_eq!(received, Err(format!("Didn't find all {} hashes, found {}", 16, 0)));
        }
    }
}