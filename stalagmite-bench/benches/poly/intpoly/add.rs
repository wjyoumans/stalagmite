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
use stalagmite_poly::intpoly::IntPoly;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use stalagmite_bench::generate_random_coeffs;

// fn generate_random_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
//     let mut rng = SmallRng::seed_from_u64(0x1234567890ABCDEF); // Fixed seed for reproducible benchmarks
//     (0..size).map(|_| rng.random_range(1..=max_coeff)).collect()
// }

// fn generate_mixed_sign_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
//     let mut rng = SmallRng::seed_from_u64(0xFEDCBA0987654321); // Different seed for variety
//     (0..size).map(|_| {
//         let val = rng.random_range(1..=max_coeff);
//         if rng.gen_bool(0.5) { val } else { -val }
//     }).collect()
// }

// Large coefficient benchmarks removed due to complexity

// ========== BASIC ADDITION BENCHMARKS ==========

fn bench_add_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Add - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [2, 10, 100, 1000, 10000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, -max_coeff,max_coeff);
        let coeffs_b = generate_random_coeffs(size, -max_coeff, max_coeff);
        
        let poly_a = IntPoly::from(coeffs_a);
        let poly_b = IntPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("owned_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(a, b)| black_box(a + b)
            )
        });
        
        group.bench_function(BenchmarkId::new("owned_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |a| black_box(a + &poly_b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_owned", size), |b| {
            b.iter_with_setup(
                || poly_b.clone(),
                |b| black_box(&poly_a + b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_ref", size), |b| {
            b.iter(|| black_box(&poly_a + &poly_b))
        });
    }
    group.finish();
}

fn bench_add_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Add - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (1, 100),
        (10, 100),
        (10, 1000),
        (100, 1000),
    ];
    let max_coeff = 1000;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, -max_coeff, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, -max_coeff, max_coeff);
        
        let poly_a = IntPoly::from(coeffs_a);
        let poly_b = IntPoly::from(coeffs_b);
        
        let bench_name = format!("{}+{}", size_a, size_b);
        
        group.bench_function(BenchmarkId::new("owned_owned", &bench_name), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(a, b)| black_box(a + b)
            )
        });
        
        group.bench_function(BenchmarkId::new("ref_ref", &bench_name), |b| {
            b.iter(|| black_box(&poly_a + &poly_b))
        });
        
        // Test the reverse as well
        let bench_name_rev = format!("{}+{}", size_b, size_a);
        group.bench_function(BenchmarkId::new("rev_ref_ref", &bench_name_rev), |b| {
            b.iter(|| black_box(&poly_b + &poly_a))
        });
    }
    group.finish();
}

// ========== ADD-ASSIGN BENCHMARKS ==========

fn bench_add_assign_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly AddAssign - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [2, 10, 100, 1000, 10000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, -max_coeff, max_coeff);
        let coeffs_b = generate_random_coeffs(size, -max_coeff, max_coeff);
        
        let poly_a = IntPoly::from(coeffs_a);
        let poly_b = IntPoly::from(coeffs_b);
        
        group.bench_function(BenchmarkId::new("assign_owned", size), |b| {
            b.iter_with_setup(
                || (poly_a.clone(), poly_b.clone()),
                |(mut a, b)| {
                    a += b;
                    black_box(a)
                }
            )
        });
        
        group.bench_function(BenchmarkId::new("assign_ref", size), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |mut a| {
                    a += &poly_b;
                    black_box(a)
                }
            )
        });
    }
    group.finish();
}

fn bench_add_assign_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly AddAssign - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (1, 100),
        (10, 100),
        (10, 1000),
        (100, 1000),
    ];
    let max_coeff = 1000;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, -max_coeff, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, -max_coeff, max_coeff);
        
        let poly_a = IntPoly::from(coeffs_a);
        let poly_b = IntPoly::from(coeffs_b);
        
        let bench_name = format!("{}+={}", size_a, size_b);
        
        group.bench_function(BenchmarkId::new("assign_ref", &bench_name), |b| {
            b.iter_with_setup(
                || poly_a.clone(),
                |mut a| {
                    a += &poly_b;
                    black_box(a)
                }
            )
        });
        
        // Test the reverse case (larger += smaller)
        let bench_name_rev = format!("{}+={}", size_b, size_a);
        group.bench_function(BenchmarkId::new("assign_ref_rev", &bench_name_rev), |b| {
            b.iter_with_setup(
                || poly_b.clone(),
                |mut b| {
                    b += &poly_a;
                    black_box(b)
                }
            )
        });
    }
    group.finish();
}

// ========== SUM BENCHMARKS ==========

fn bench_sum_small_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Sum - small polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000, 5000];
    let poly_size = 5;
    let max_coeff = 100;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, -max_coeff, max_coeff))
            .collect();
        
        let polys: Vec<IntPoly> = coeffs_data.iter()
            .map(|coeffs| IntPoly::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("owned", n), |b| {
            b.iter(|| {
                let polys_clone = polys.clone();
                black_box(polys_clone.into_iter().sum::<IntPoly>())
            })
        });
        
        group.bench_function(BenchmarkId::new("ref", n), |b| {
            b.iter(|| black_box(polys.iter().sum::<IntPoly>()))
        });
    }
    group.finish();
}

fn bench_sum_medium_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Sum - medium polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000];
    let poly_size = 100;
    let max_coeff = 1000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, -max_coeff, max_coeff))
            .collect();
        
        let polys: Vec<IntPoly> = coeffs_data.iter()
            .map(|coeffs| IntPoly::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("owned", n), |b| {
            b.iter(|| {
                let polys_clone = polys.clone();
                black_box(polys_clone.into_iter().sum::<IntPoly>())
            })
        });
        
        group.bench_function(BenchmarkId::new("ref", n), |b| {
            b.iter(|| black_box(polys.iter().sum::<IntPoly>()))
        });
    }
    group.finish();
}

fn bench_sum_large_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Sum - medium polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100];
    let poly_size = 1000;
    let max_coeff = 10000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, -max_coeff, max_coeff))
            .collect();
        
        let polys: Vec<IntPoly> = coeffs_data.iter()
            .map(|coeffs| IntPoly::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("owned", n), |b| {
            b.iter(|| {
                let polys_clone = polys.clone();
                black_box(polys_clone.into_iter().sum::<IntPoly>())
            })
        });
        
        group.bench_function(BenchmarkId::new("ref", n), |b| {
            b.iter(|| black_box(polys.iter().sum::<IntPoly>()))
        });
    }
    group.finish();
}

fn bench_sum_varying_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly Sum - varying polynomial sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = 100;
    let poly_sizes = [1, 10, 50, 100, 200];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..num_polys)
            .map(|_| generate_random_coeffs(size, -max_coeff, max_coeff))
            .collect();
        
        let polys: Vec<IntPoly> = coeffs_data.iter()
            .map(|coeffs| IntPoly::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("owned", size), |b| {
            b.iter(|| {
                let polys_clone = polys.clone();
                black_box(polys_clone.into_iter().sum::<IntPoly>())
            })
        });
        
        group.bench_function(BenchmarkId::new("ref", size), |b| {
            b.iter(|| black_box(polys.iter().sum::<IntPoly>()))
        });
    }
    group.finish();
}

// ========== INTPOLY ADDITION BENCHMARKS ==========

criterion_group! {
    name = addition_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_add_same_size,
        bench_add_different_sizes,

}

criterion_group! {
    name = add_assign_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_add_assign_same_size,
        bench_add_assign_different_sizes
}

criterion_group! {
    name = sum_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_sum_small_polys,
        bench_sum_medium_polys,
        bench_sum_large_polys,
        bench_sum_varying_sizes
}

criterion_main!(
    addition_benches,
    add_assign_benches, 
    sum_benches
);
