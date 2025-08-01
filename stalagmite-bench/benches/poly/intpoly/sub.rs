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
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;


fn generate_random_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(0x1234567890ABCDEF); // Fixed seed for reproducible benchmarks
    (0..size).map(|_| rng.random_range(1..=max_coeff)).collect()
}

fn generate_mixed_sign_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(0xFEDCBA0987654321); // Different seed for variety
    (0..size).map(|_| {
        let val = rng.random_range(1..=max_coeff);
        if rng.gen_bool(0.5) { val } else { -val }
    }).collect()
}

// Large coefficient benchmarks removed due to complexity

// ========== BASIC SUBTRACTION BENCHMARKS ==========

fn bench_sub_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Sub - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 5, 10, 50, 100, 500, 1000, 2000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, max_coeff);
        let coeffs_b = generate_random_coeffs(size, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("owned_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(a, b)| black_box(a - b)
            )
        });
        
        group.bench_function(BenchmarkId::new("owned_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |a| black_box(a - &poly_b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_owned", size), |b| {
            b.iter_with_setup(
                || poly_b.clone(),
                |b| black_box(&poly_a - b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_ref", size), |b| {
            b.iter(|| black_box(&poly_a - &poly_b))
        });
    }
    group.finish();
}

fn bench_sub_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Sub - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (1usize, 10usize),
        (5, 50),
        (10, 100),
        (50, 500),
        (100, 1000),
        (10, 1000),
        (1, 1000),
    ];
    let max_coeff = 1000;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        let bench_name = format!("{}-{}", size_a, size_b);
        
        group.bench_function(BenchmarkId::new("owned_owned", &bench_name), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(a, b)| black_box(a - b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_ref", &bench_name), |b| {
            b.iter(|| black_box(&poly_a - &poly_b))
        });
        
        // Test the reverse as well
        let bench_name_rev = format!("{}-{}", size_b, size_a);
        group.bench_function(BenchmarkId::new("rev_ref_ref", &bench_name_rev), |b| {
            b.iter(|| black_box(&poly_b - &poly_a))
        });
    }
    group.finish();
}

fn bench_sub_with_zero(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Sub - with zero");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 10, 50, 100, 500, 1000];
    let max_coeff = 1000;
    let zero = ZZPoly::zero();
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("poly_minus_zero", size), |b| {
            b.iter(|| black_box(&poly - &zero))
        });
        
        group.bench_function(BenchmarkId::new("zero_minus_poly", size), |b| {
            b.iter(|| black_box(&zero - &poly))
        });
        
        group.bench_function(BenchmarkId::new("poly_minus_zero_owned", size), |b| {
            b.iter_with_setup(
                || (poly.clone(), zero.clone()),
                |(p, z)| black_box(p - z)
            )
        });
    }
    group.finish();
}

fn bench_sub_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Sub - self subtraction");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 10, 50, 100, 500, 1000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("self_minus_self", size), |b| {
            b.iter(|| black_box(&poly - &poly))
        });
        
        group.bench_function(BenchmarkId::new("self_minus_self_owned", size), |b| {
            b.iter_with_setup(
                || (poly.clone(), poly.clone()),
                |(a, b)| black_box(a - b)
            )
        });
    }
    group.finish();
}

// Large coefficient benchmarks removed for simplicity

// ========== SUB-ASSIGN BENCHMARKS ==========

fn bench_sub_assign_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly SubAssign - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 5, 10, 50, 100, 500, 1000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, max_coeff);
        let coeffs_b = generate_random_coeffs(size, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("assign_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(mut a, b)| {
                    a -= b;
                    black_box(a)
                }
            )
        });
        
        group.bench_function(BenchmarkId::new("assign_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |mut a| {
                    a -= &poly_b;
                    black_box(a)
                }
            )
        });
    }
    group.finish();
}

fn bench_sub_assign_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly SubAssign - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (10usize, 100usize),
        (50, 500),
        (100, 1000),
        (1, 1000),
    ];
    let max_coeff = 1000;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        let bench_name = format!("{}-={}", size_a, size_b);
        
        group.bench_function(BenchmarkId::new("assign_ref", &bench_name), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |mut a| {
                    a -= &poly_b;
                    black_box(a)
                }
            )
        });
        
        // Test the reverse case (larger -= smaller)
        let bench_name_rev = format!("{}-={}", size_b, size_a);
        group.bench_function(BenchmarkId::new("assign_ref_rev", &bench_name_rev), |b| {
            b.iter_with_setup(
                || poly_b.clone(),
                |mut b| {
                    b -= &poly_a;
                    black_box(b)
                }
            )
        });
    }
    group.finish();
}

// ========== INTEGER SUBTRACTION BENCHMARKS ==========

// Integer scalar benchmarks removed due to type issues

criterion_group! {
    name = subtraction_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 
        bench_sub_same_size,
        bench_sub_different_sizes,
        bench_sub_with_zero,
        bench_sub_self,

}

criterion_group! {
    name = sub_assign_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 
        bench_sub_assign_same_size,
        bench_sub_assign_different_sizes
}

// Integer benchmarks removed

criterion_main!(
    subtraction_benches,
    sub_assign_benches
);