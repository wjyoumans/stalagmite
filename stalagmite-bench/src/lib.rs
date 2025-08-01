
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

const BENCH_SEED: u64 = 0x1234567890ABCDEF;

pub fn generate_random_coeffs(size: usize, min_coeff: i32, max_coeff: i32) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(BENCH_SEED);
    (0..size).map(|_| rng.random_range(min_coeff..=max_coeff)).collect()
}

// fn generate_mixed_sign_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
//     let mut rng = SmallRng::seed_from_u64(0xFEDCBA0987654321); // Different seed for variety
//     (0..size).map(|_| {
//         let val = rng.random_range(1..=max_coeff);
//         if rng.gen_bool(0.5) { val } else { -val }
//     }).collect()
// }
