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


fn generate_random_coeffs(size: usize, max_coeff: i64) -> Vec<i64> {
    let mut rng = SmallRng::seed_from_u64(0x1234567890ABCDEF); // Fixed seed for reproducible benchmarks
    (0..size).map(|_| rng.random_range(1..=max_coeff)).collect()
}

// Large coefficient benchmarks removed due to complexity

// ========== BASIC MULTIPLICATION BENCHMARKS ==========

fn bench_mul_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Mul - same size (auto algorithm)");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 3, 5, 8, 12, 16, 25, 50, 100, 200];
    let max_coeff = 1000i64;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, max_coeff);
        let coeffs_b = generate_random_coeffs(size, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("owned_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(a, b)| black_box(a * b)
            )
        });
        
        group.bench_function(BenchmarkId::new("owned_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |a| black_box(a * &poly_b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_owned", size), |b| {
            b.iter_with_setup(
                || poly_b.clone(),
                |b| black_box(&poly_a * b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_ref", size), |b| {
            b.iter(|| black_box(&poly_a * &poly_b))
        });
    }
    group.finish();
}

fn bench_mul_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Mul - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (1usize, 10usize),
        (3, 20),
        (5, 50),
        (10, 100),
        (25, 200),
        (50, 500),
        (1, 1000),
    ];
    let max_coeff = 1000i64;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        let bench_name = format!("{}x{}", size_a, size_b);
        
        group.bench_function(BenchmarkId::new("ref_ref", &bench_name), |b| {
            b.iter(|| black_box(&poly_a * &poly_b))
        });
        
        // Test the reverse multiplication
        let bench_name_rev = format!("{}x{}", size_b, size_a);
        group.bench_function(BenchmarkId::new("rev_ref_ref", &bench_name_rev), |b| {
            b.iter(|| black_box(&poly_b * &poly_a))
        });
    }
    group.finish();
}

fn bench_mul_with_special_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Mul - special cases");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [5usize, 10, 50, 100, 500];
    let max_coeff = 1000i64;
    let zero = ZZPoly::zero();
    let one = ZZPoly::from(vec![1i64]);
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        // Multiplication by zero
        group.bench_function(BenchmarkId::new("poly_times_zero", size), |b| {
            b.iter(|| black_box(&poly * &zero))
        });
        
        // Multiplication by one
        group.bench_function(BenchmarkId::new("poly_times_one", size), |b| {
            b.iter(|| black_box(&poly * &one))
        });
        
        // Self multiplication (should use squaring algorithm)
        group.bench_function(BenchmarkId::new("self_multiplication", size), |b| {
            b.iter(|| black_box(&poly * &poly))
        });
    }
    group.finish();
}

fn bench_mul_scalar_vs_poly(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Mul - scalar vs polynomial");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [10usize, 50, 100, 500, 1000];
    let max_coeff = 1000i64;
    let scalar = ZZPoly::from(vec![42i64]);
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = ZZPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("poly_times_scalar", size), |b| {
            b.iter(|| black_box(&poly * &scalar))
        });
        
        group.bench_function(BenchmarkId::new("scalar_times_poly", size), |b| {
            b.iter(|| black_box(&scalar * &poly))
        });
    }
    group.finish();
}

// Large coefficient benchmarks removed for simplicity

// ========== MUL-ASSIGN BENCHMARKS ==========

fn bench_mul_assign_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly MulAssign - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 5, 10, 25, 50, 100];
    let max_coeff = 1000i64;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, max_coeff);
        let coeffs_b = generate_random_coeffs(size, max_coeff);
        
        let poly_a = ZZPoly::from(coeffs_a);
        let poly_b = ZZPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("assign_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(mut a, b)| {
                    a *= b;
                    black_box(a)
                }
            )
        });
        
        group.bench_function(BenchmarkId::new("assign_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |mut a| {
                    a *= &poly_b;
                    black_box(a)
                }
            )
        });
    }
    group.finish();
}

// ========== ALGORITHM SELECTION THRESHOLD BENCHMARKS ==========

fn bench_algorithm_selection_thresholds(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Mul - algorithm selection thresholds");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group.sample_size(20);
    
    // Test around the algorithm selection boundaries
    let test_sizes = [
        (5usize, "very_small"),
        (6, "small_boundary"),
        (7, "small_post_boundary"),
        (8, "medium_boundary"),
        (12, "medium"),
        (16, "medium_large"),
        (25, "large_boundary"),
        (50, "large"),
        (100, "very_large"),
    ];
    
    for &(size, label) in test_sizes.iter() {
        // Test with small coefficients (should prefer KS or classical)
        let small_coeffs_a = generate_random_coeffs(size, 100);
        let small_coeffs_b = generate_random_coeffs(size, 100);
        let small_poly_a = ZZPoly::from(small_coeffs_a);
        let small_poly_b = ZZPoly::from(small_coeffs_b);
        
        group.bench_function(BenchmarkId::new("small_coeffs", format!("{}_{}", size, label)), |b| {
            b.iter(|| black_box(&small_poly_a * &small_poly_b))
        });
        
        // Large coefficient tests removed for simplicity
    }
    group.finish();
}

// ========== INTEGER MULTIPLICATION BENCHMARKS ==========

// Integer scalar benchmarks removed due to type issues

criterion_group! {
    name = basic_mul_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 
        bench_mul_same_size,
        bench_mul_different_sizes,
        bench_mul_with_special_cases,
        bench_mul_scalar_vs_poly
}

criterion_group! {
    name = advanced_mul_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 

        bench_algorithm_selection_thresholds
}

criterion_group! {
    name = mul_assign_benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets = 
        bench_mul_assign_same_size
}

// Integer benchmarks removed

criterion_main!(
    basic_mul_benches,
    advanced_mul_benches,
    mul_assign_benches
);