use stalagmite_zn::{ZnRing, ZnElem};
use malachite::Natural;
use malachite::base::num::random::{RandomPrimitiveInts, random_primitive_ints};
use malachite::base::random::EXAMPLE_SEED;

pub fn main() {
    let mut rng = random_primitive_ints(EXAMPLE_SEED);
    let ring = ZnRing::init(Natural::from(1000u32));
    let elements = ring.random_elements(&mut rng, 100);
    println!("{:?}", elements);
}