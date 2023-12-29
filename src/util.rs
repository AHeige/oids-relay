use rand::Rng;

pub fn generate_random_id() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..10000)
    }
