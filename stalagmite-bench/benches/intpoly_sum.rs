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
extern crate stalagmite_poly2;

use criterion::*;

// Type aliases to distinguish between the two IntPoly implementations
type IntPolyV1 = stalagmite_poly::intpoly::IntPoly;
type IntPolyV2 = stalagmite_poly2::intpoly::IntPoly;

fn generate_random_coeffs(size: usize, max_coeff: i32) -> Vec<i32> {
    (0..size).map(|i| (i as i32 % max_coeff) + 1).collect()
}

fn bench_sum_v1_owned(polys: &[IntPolyV1]) -> IntPolyV1 {
    polys.iter().cloned().sum()
}

fn bench_sum_v1_ref(polys: &[IntPolyV1]) -> IntPolyV1 {
    polys.iter().sum()
}

fn bench_sum_v2_owned(polys: &[IntPolyV2]) -> IntPolyV2 {
    polys.iter().cloned().sum()
}

fn bench_sum_v2_ref(polys: &[IntPolyV2]) -> IntPolyV2 {
    polys.iter().sum()
}

fn bench_sum_small_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly sum - small polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000, 10000];
    let poly_size = 5; // Small polynomials
    let max_coeff = 100;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
            .collect();
        
        let polys_v1: Vec<IntPolyV1> = coeffs_data.iter()
            .map(|coeffs| IntPolyV1::from(coeffs.clone()))
            .collect();
        
        let polys_v2: Vec<IntPolyV2> = coeffs_data.iter()
            .map(|coeffs| IntPolyV2::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_owned", n), |b| {
            b.iter(|| bench_sum_v1_owned(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_ref", n), |b| {
            b.iter(|| bench_sum_v1_ref(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_owned", n), |b| {
            b.iter(|| bench_sum_v2_owned(black_box(&polys_v2)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_ref", n), |b| {
            b.iter(|| bench_sum_v2_ref(black_box(&polys_v2)))
        });
    }
    group.finish();
}

fn bench_sum_medium_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly sum - medium polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100, 1000];
    let poly_size = 50; // Medium polynomials
    let max_coeff = 1000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
            .collect();
        
        let polys_v1: Vec<IntPolyV1> = coeffs_data.iter()
            .map(|coeffs| IntPolyV1::from(coeffs.clone()))
            .collect();
        
        let polys_v2: Vec<IntPolyV2> = coeffs_data.iter()
            .map(|coeffs| IntPolyV2::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_owned", n), |b| {
            b.iter(|| bench_sum_v1_owned(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_ref", n), |b| {
            b.iter(|| bench_sum_v1_ref(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_owned", n), |b| {
            b.iter(|| bench_sum_v2_owned(black_box(&polys_v2)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_ref", n), |b| {
            b.iter(|| bench_sum_v2_ref(black_box(&polys_v2)))
        });
    }
    group.finish();
}

fn bench_sum_large_polys(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntPoly sum - large polynomials");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let num_polys = [10u64, 100];
    let poly_size = 500; // Large polynomials
    let max_coeff = 10000;
    
    for &n in num_polys.iter() {
        let coeffs_data: Vec<Vec<i32>> = (0..n)
            .map(|_| generate_random_coeffs(poly_size, max_coeff))
            .collect();
        
        let polys_v1: Vec<IntPolyV1> = coeffs_data.iter()
            .map(|coeffs| IntPolyV1::from(coeffs.clone()))
            .collect();
        
        let polys_v2: Vec<IntPolyV2> = coeffs_data.iter()
            .map(|coeffs| IntPolyV2::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_owned", n), |b| {
            b.iter(|| bench_sum_v1_owned(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_ref", n), |b| {
            b.iter(|| bench_sum_v1_ref(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_owned", n), |b| {
            b.iter(|| bench_sum_v2_owned(black_box(&polys_v2)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_ref", n), |b| {
            b.iter(|| bench_sum_v2_ref(black_box(&polys_v2)))
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
        
        let polys_v1: Vec<IntPolyV1> = coeffs_data.iter()
            .map(|coeffs| IntPolyV1::from(coeffs.clone()))
            .collect();
        
        let polys_v2: Vec<IntPolyV2> = coeffs_data.iter()
            .map(|coeffs| IntPolyV2::from(coeffs.clone()))
            .collect();
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_owned", size), |b| {
            b.iter(|| bench_sum_v1_owned(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly_ref", size), |b| {
            b.iter(|| bench_sum_v1_ref(black_box(&polys_v1)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_owned", size), |b| {
            b.iter(|| bench_sum_v2_owned(black_box(&polys_v2)))
        });
        
        group.bench_function(BenchmarkId::new("stalagmite_poly2_ref", size), |b| {
            b.iter(|| bench_sum_v2_ref(black_box(&polys_v2)))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_sum_small_polys,
        bench_sum_medium_polys, 
        bench_sum_large_polys,
        bench_sum_varying_sizes
}
criterion_main!(benches);