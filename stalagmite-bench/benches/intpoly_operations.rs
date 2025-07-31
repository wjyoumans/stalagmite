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

fn generate_random_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    (0..size).map(|i| (i as i32 % max_coeff) + 1).collect()
}

// ========== ADDITION BENCHMARKS ==========

fn bench_add_same_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly addition - same size");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 5, 10, 50, 100, 500, 1000];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_a = generate_random_coeffs(size, max_coeff);
        let coeffs_b = generate_random_coeffs(size, max_coeff);
        
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
    let mut group = c.benchmark_group("IntPoly addition - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (1usize, 10usize),
        (5, 50),
        (10, 100),
        (50, 500),
        (100, 1000),
    ];
    let max_coeff = 1000;
    
    for &(size_a, size_b) in size_pairs.iter() {
        let coeffs_a = generate_random_coeffs(size_a, max_coeff);
        let coeffs_b = generate_random_coeffs(size_b, max_coeff);
        
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
    }
    group.finish();
}

fn bench_add_with_zero(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly addition - with zero");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let poly_sizes = [1usize, 10, 50, 100, 500];
    let max_coeff = 1000;
    let zero = IntPoly::zero();
    
    for &size in poly_sizes.iter() {
        let coeffs = generate_random_coeffs(size, max_coeff);
        let poly = IntPoly::from(coeffs);
        
        group.bench_function(BenchmarkId::new("poly_plus_zero", size), |b| {
            b.iter(|| black_box(&poly + &zero))
        });
        
        group.bench_function(BenchmarkId::new("zero_plus_poly", size), |b| {
            b.iter(|| black_box(&zero + &poly))
        });
    }
    group.finish();
}

// ========== SUM BENCHMARKS ==========

fn bench_sum_small_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly sum - small polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000, 10000];
    let poly_size = 5;
    let max_coeff = 100;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
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
    let mut group = c.benchmark_group("IntPoly sum - medium polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000];
    let poly_size = 50;
    let max_coeff = 1000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
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
    let mut group = c.benchmark_group("IntPoly sum - large polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100];
    let poly_size = 500;
    let max_coeff = 10000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
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
    let mut group = c.benchmark_group("IntPoly sum - varying polynomial sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = 100;
    let poly_sizes = [1usize, 2, 5, 10, 20, 50, 100, 200];
    let max_coeff = 1000;
    
    for &size in poly_sizes.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..num_polys)
            .map(|_| generate_random_coeffs(size, max_coeff))
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

criterion_group! {
    name = addition_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_add_same_size,
        bench_add_different_sizes,
        bench_add_with_zero
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

criterion_main!(addition_benches, sum_benches);