// Copyright (C) 2025 William Youmans
//
// This file is part of Stalagmite.
//
// Stalagmite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Stalagmite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stalagmite. If not, see <https://www.gnu.org/licenses/>.

extern crate criterion;
extern crate stalagmite_zn;
extern crate stalagmite_bench;
extern crate malachite;
extern crate rand;

use criterion::*;
use malachite::Natural;
use malachite::natural::random::get_random_natural_with_up_to_bits;
use malachite::base::num::random::{RandomPrimitiveInts, random_primitive_ints};
use stalagmite_bench::BENCH_SEED;
use stalagmite_zn::{ZnRing, ZnElem};

/// Generate a random modulus with the specified number of bits
fn random_modulus(rng: &mut RandomPrimitiveInts<u64>, bits: u64) -> Natural {
    let mut modulus = get_random_natural_with_up_to_bits(rng, bits);
    if modulus <= Natural::from(1u32) {
        modulus = Natural::from(3u32);
    }
    modulus
}

fn bench_mul_word_size_moduli(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_word_size");
    let mut rng = random_primitive_ints(BENCH_SEED);

    // Test various word-size moduli
    for bits in [32, 48, 63] {
        let modulus = random_modulus(&mut rng, bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = ring.random_elements(&mut rng, 100);
         
        group.bench_function(
            &format!("{}_bits (modulus={})", bits, modulus),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a * b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_mul_large_moduli(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_large");
    let mut rng = random_primitive_ints(BENCH_SEED);
    
    // Test large moduli
    for bits in [256, 512, 1024, 2048] {
        let modulus = random_modulus(&mut rng, bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = ring.random_elements(&mut rng, 100);
        
        group.bench_function(
            &format!("{}_bits (modulus={})", bits, modulus),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a * b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_mul_assign_vs_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_assign_vs_mul");
    
    let mut rng = random_primitive_ints(BENCH_SEED);
    let modulus = random_modulus(&mut rng, 1024);
    let ring = ZnRing::init(modulus.clone());
    let elements = ring.random_elements(&mut rng, 100);
    
    group.bench_function(&format!("mul_assign (modulus={})", modulus), |b| {
        let mut i = 0;
        b.iter(|| {
            let mut a = elements[i % elements.len()].clone();
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            a *= b;
            black_box(a)
        })
    });
    
    group.bench_function(&format!("mul (modulus={})", modulus), |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a * b)
        })
    });
    
    group.finish();
}

fn bench_mul_reference_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_reference_patterns");
    
    let mut rng = random_primitive_ints(BENCH_SEED);
    let modulus = random_modulus(&mut rng, 512);
    let ring = ZnRing::init(modulus.clone());
    let elements = ring.random_elements(&mut rng, 100);
    
    group.bench_function("owned_owned", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = elements[i % elements.len()].clone();
            let b = elements[(i + 1) % elements.len()].clone();
            i += 2;
            black_box(a * b)
        })
    });
    
    group.bench_function("owned_ref", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = elements[i % elements.len()].clone();
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a * b)
        })
    });
    
    group.bench_function("ref_owned", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = elements[(i + 1) % elements.len()].clone();
            i += 2;
            black_box(a * b)
        })
    });
    
    group.bench_function("ref_ref", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a * b)
        })
    });
    
    group.finish();
}

fn bench_mul_precomputed_data_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_precomputed_vs_naive");
    let mut rng = random_primitive_ints(BENCH_SEED);
    
    // Compare with different modulus sizes to see the impact of precomputed data
    for bits in [64, 256, 1024] {
        let modulus = random_modulus(&mut rng, bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = ring.random_elements(&mut rng, 100);
        
        group.bench_function(
            &format!("precomputed_{}_bits", bits),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a * b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_mul_repeated_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_repeated");
    
    let mut rng = random_primitive_ints(BENCH_SEED);
    let modulus = random_modulus(&mut rng, 256);
    let ring = ZnRing::init(modulus.clone());
    let base = ring.new(Natural::from(3u32));
    
    // Benchmark repeated multiplication (like exponentiation)
    for count in [10, 100, 1000] {
        group.bench_function(
            &format!("power_{}_multiplications", count),
            |b| {
                b.iter(|| {
                    let mut result = ring.new(Natural::from(1u32));
                    for _ in 0..count {
                        result *= &base;
                    }
                    black_box(result)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_mul_modulus_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_modulus_scaling");
    let mut rng = random_primitive_ints(BENCH_SEED);
    
    // Test how performance scales with modulus size
    for bits in [64, 128, 256, 512, 1024, 2048] {
        let modulus = random_modulus(&mut rng, bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = ring.random_elements(&mut rng, 50);
        
        group.throughput(Throughput::Elements(1));
        group.bench_function(
            &format!("{}_bits", bits),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a * b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_mul_special_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_mul_special_cases");
    
    let mut rng = random_primitive_ints(BENCH_SEED);
    let modulus = random_modulus(&mut rng, 512);
    let ring = ZnRing::init(modulus.clone());
    let elements = ring.random_elements(&mut rng, 100);
    let zero = ring.new(Natural::from(0u32));
    let one = ring.new(Natural::from(1u32));
    
    group.bench_function("multiply_by_zero", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            i += 1;
            black_box(a * &zero)
        })
    });
    
    group.bench_function("multiply_by_one", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            i += 1;
            black_box(a * &one)
        })
    });
    
    group.bench_function("square", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            i += 1;
            black_box(a * a)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_mul_word_size_moduli,
    bench_mul_large_moduli,
    bench_mul_assign_vs_mul,
    bench_mul_reference_patterns,
    bench_mul_precomputed_data_impact,
    bench_mul_repeated_operations,
    bench_mul_modulus_size_scaling,
    bench_mul_special_cases
);
criterion_main!(benches);
