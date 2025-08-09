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
extern crate malachite;
extern crate rand;

use criterion::*;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Parity;
use rand::{Rng, SeedableRng, rngs::StdRng};
use stalagmite_zn::{ZnRing, ZnElem};

/// Generate a random modulus with the specified number of bits
fn random_modulus(bits: u64) -> Natural {
    let mut rng = StdRng::seed_from_u64(42);
    // Ensure modulus is odd and greater than 1
    let mut modulus = random_natural_with_bits(&mut rng, bits);
    if modulus <= Natural::from(1u32) {
        modulus = Natural::from(3u32);
    }
    if modulus.even() {
        modulus += Natural::from(1u32);
    }
    modulus
}

/// Generate random elements in Zn
fn generate_random_elements(ring: &ZnRing, count: usize) -> Vec<ZnElem> {
    let mut rng = StdRng::seed_from_u64(123);
    
    (0..count)
        .map(|_| {
            let value: u64 = rng.gen();
            ring.new(Natural::from(value) % ring.modulus())
        })
        .collect()
}

fn bench_add_word_size_moduli(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_word_size");
    
    // Test various word-size moduli
    for bits in [32, 48, 63] {
        let modulus = random_modulus(bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = generate_random_elements(&ring, 1000);
        
        group.bench_function(
            &format!("{}_bits", bits),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a + b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_add_large_moduli(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_large");
    
    // Test large moduli
    for bits in [256, 512, 1024, 2048] {
        let modulus = random_modulus(bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = generate_random_elements(&ring, 100);
        
        group.bench_function(
            &format!("{}_bits", bits),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a + b)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_add_assign_vs_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_assign_vs_add");
    
    let modulus = random_modulus(1024);
    let ring = ZnRing::init(modulus.clone());
    let elements = generate_random_elements(&ring, 100);
    
    group.bench_function("add_assign", |b| {
        let mut i = 0;
        b.iter(|| {
            let mut a = elements[i % elements.len()].clone();
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            a += b;
            black_box(a)
        })
    });
    
    group.bench_function("add", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a + b)
        })
    });
    
    group.finish();
}

fn bench_add_reference_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_reference_patterns");
    
    let modulus = random_modulus(512);
    let ring = ZnRing::init(modulus.clone());
    let elements = generate_random_elements(&ring, 100);
    
    group.bench_function("owned_owned", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = elements[i % elements.len()].clone();
            let b = elements[(i + 1) % elements.len()].clone();
            i += 2;
            black_box(a + b)
        })
    });
    
    group.bench_function("owned_ref", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = elements[i % elements.len()].clone();
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a + b)
        })
    });
    
    group.bench_function("ref_owned", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = elements[(i + 1) % elements.len()].clone();
            i += 2;
            black_box(a + b)
        })
    });
    
    group.bench_function("ref_ref", |b| {
        let mut i = 0;
        b.iter(|| {
            let a = &elements[i % elements.len()];
            let b = &elements[(i + 1) % elements.len()];
            i += 2;
            black_box(a + b)
        })
    });
    
    group.finish();
}

fn bench_add_repeated_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_repeated");
    
    let modulus = random_modulus(256);
    let ring = ZnRing::init(modulus.clone());
    let elements = generate_random_elements(&ring, 10);
    
    for count in [10, 100, 1000] {
        group.bench_function(
            &format!("sum_{}_elements", count),
            |b| {
                b.iter(|| {
                    let mut sum = ring.new(Natural::from(0u32));
                    for i in 0..count {
                        sum += &elements[i % elements.len()];
                    }
                    black_box(sum)
                })
            }
        );
    }
    
    group.finish();
}

fn bench_add_modulus_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("zn_add_modulus_scaling");
    
    // Test how performance scales with modulus size
    for bits in [64, 128, 256, 512, 1024] {
        let modulus = random_modulus(bits);
        let ring = ZnRing::init(modulus.clone());
        let elements = generate_random_elements(&ring, 50);
        
        group.throughput(Throughput::Elements(1));
        group.bench_function(
            &format!("{}_bits", bits),
            |b| {
                let mut i = 0;
                b.iter(|| {
                    let a = &elements[i % elements.len()];
                    let b = &elements[(i + 1) % elements.len()];
                    i += 2;
                    black_box(a + b)
                })
            }
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_add_word_size_moduli,
    bench_add_large_moduli,
    bench_add_assign_vs_add,
    bench_add_reference_patterns,
    bench_add_repeated_operations,
    bench_add_modulus_size_scaling
);
criterion_main!(benches);
