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
extern crate stalagmite_poly;

use criterion::*;
use stalagmite_poly::zz_poly::ZZPoly;
use malachite::base::num::arithmetic::traits::NegAssign;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

fn generate_random_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(0x1234567890ABCDEF); // Fixed seed for reproducible benchmarks
    (0..size).map(|_| rng.random_range(1..=max_coeff)).collect()
}

// Large coefficient benchmarks removed due to complexity - use simple coefficients instead

fn generate_mixed_sign_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(0xFEDCBA0987654321); // Different seed for variety
    (0..size).map(|_| {
        let val = rng.random_range(1..=max_coeff);
        if rng.gen_bool(0.5) { val } else { -val }
    }).collect()
}

// ========== BASIC NEGATION BENCHMARKS ==========

fn bench_neg_owned_vs_ref(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - owned vs reference");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 5, 10, 50, 100, 500, 1000, 2000, 5000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("neg_owned", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |p| black_box(-p)
            )
        });
        
        group.bench_function(BenchmarkId::new("neg_ref", size), |b| {
            b.iter(|| black_box(-&poly))
        });
    }
    group.finish();
}

fn bench_neg_vs_neg_assign(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - negation vs neg_assign");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 10, 50, 100, 500, 1000, 2000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("neg_owned", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |p| black_box(-p)
            )
        });
        
        group.bench_function(BenchmarkId::new("neg_assign", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |mut p| {
                    p.neg_assign();
                    black_box(p)
                }
            )
        });
        
        group.bench_function(BenchmarkId::new("neg_ref", size), |b| {
            b.iter(|| black_box(-&poly))
        });
    }
    group.finish();
}

// Large coefficient benchmarks removed for simplicity

fn bench_neg_mixed_signs(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - mixed sign coefficients");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [10usize, 50, 100, 500, 1000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let positive_coeffs = generate_random_coeffs(size, max_coeff);
        let mixed_coeffs = generate_mixed_sign_coeffs(size, max_coeff);
        
        let positive_poly = ZZPoly::from(positive_coeffs);
        let mixed_poly = ZZPoly::from(mixed_coeffs);
        
        group.bench_function(BenchmarkId::new("neg_positive", size), |b| {
            b.iter(|| black_box(-&positive_poly))
        });
        
        group.bench_function(BenchmarkId::new("neg_mixed", size), |b| {
            b.iter(|| black_box(-&mixed_poly))
        });
    }
    group.finish();
}

fn bench_neg_special_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - special cases");
    
    let sizes = [1usize, 10, 100, 1000];
    let max_coeff = 1000;
    let zero = ZZPoly::zero();
    
    for &size in sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        // Zero polynomial negation
        group.bench_function(BenchmarkId::new("neg_zero", size), |b| {
            b.iter(|| black_box(-&zero))
        });
        
        // Constant polynomial negation
        if size == 1 {
            group.bench_function(BenchmarkId::new("neg_constant", size), |b| {
                b.iter(|| black_box(-&poly))
            });
        }
    }
    
    // Dense vs sparse polynomials (simulate sparse with zeros)
    let dense_coeffs = generate_random_coeffs(100, max_coeff);
    let mut sparse_coeffs = vec![0i32; 100];
    for i in (0..100).step_by(10) {
        sparse_coeffs[i] = (i as i32 % max_coeff) + 1;
    }
    
    let dense_poly = ZZPoly::from(dense_coeffs);
    let sparse_poly = ZZPoly::from(sparse_coeffs);
    
    group.bench_function("neg_dense_100", |b| {
        b.iter(|| black_box(-&dense_poly))
    });
    
    group.bench_function("neg_sparse_100", |b| {
        b.iter(|| black_box(-&sparse_poly))
    });
    
    group.finish();
}

fn bench_double_negation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - double negation");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [10usize, 50, 100, 500, 1000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("double_neg_ref", size), |b| {
            b.iter(|| black_box(-(-&poly)))
        });
        
        group.bench_function(BenchmarkId::new("double_neg_owned", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |p| black_box(-(-p))
            )
        });
        
        group.bench_function(BenchmarkId::new("double_neg_assign", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |mut p| {
                    p.neg_assign();
                    p.neg_assign();
                    black_box(p)
                }
            )
        });
    }
    group.finish();
}

// ========== MEMORY ALLOCATION BENCHMARKS ==========

fn bench_neg_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Neg - memory allocation patterns");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [100usize, 1000, 5000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        // Single negation (creates new polynomial)
        group.bench_function(BenchmarkId::new("single_neg_ref", size), |b| {
            b.iter(|| black_box(-&poly))
        });
        
        // In-place negation (modifies existing polynomial)
        group.bench_function(BenchmarkId::new("inplace_neg_assign", size), |b| {
            b.iter_with_setup(
                || poly.clone(),
                |mut p| {
                    p.neg_assign();
                    black_box(p)
                }
            )
        });
        
        // Chain of operations involving negation
        group.bench_function(BenchmarkId::new("neg_in_chain", size), |b| {
            b.iter(|| {
                let neg_poly = -&poly;
                black_box(&neg_poly + &poly) // Should give zero
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = basic_neg_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 
        bench_neg_owned_vs_ref,
        bench_neg_vs_neg_assign,
        bench_neg_special_cases
}

criterion_group! {
    name = advanced_neg_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 

        bench_neg_mixed_signs,
        bench_double_negation
}

criterion_group! {
    name = memory_neg_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_neg_memory_patterns
}

criterion_main!(
    basic_neg_benches,
    advanced_neg_benches,
    memory_neg_benches
);