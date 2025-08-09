use malachite::base::random::Seed;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

// malachite::base::random::EXAMPLE_SEED
pub const BENCH_SEED: Seed = Seed::from_bytes([
    0xbf, 0x18, 0x11, 0xce, 0x15, 0xee, 0xfd, 0x20, 0x2f, 0xdf, 0x67, 0x6a, 0x6b, 0xba, 0xaf, 0x04,
    0xff, 0x71, 0xe0, 0xf8, 0x0b, 0x2a, 0xcf, 0x27, 0x85, 0xb3, 0x32, 0xc6, 0x20, 0x80, 0x5e, 0x36,
]);

//pub fn generate_random_coeffs(size: usize, min_coeff: i32, max_coeff: i32) -> Vec<i32> {
//    let mut rng = SmallRng::seed_from_u64(BENCH_SEED);
//    (0..size).map(|_| rng.random_range(min_coeff..=max_coeff)).collect()
//}

// fn generate_mixed_sign_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
//     let mut rng = SmallRng::seed_from_u64(0xFEDCBA0987654321); // Different seed for variety
//     (0..size).map(|_| {
//         let val = rng.random_range(1..=max_coeff);
//         if rng.gen_bool(0.5) { val } else { -val }
//     }).collect()
// }
