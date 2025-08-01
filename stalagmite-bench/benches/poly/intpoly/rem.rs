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

// ========== REMAINDER BENCHMARKS ==========

fn bench_rem_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly Rem - different sizes");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (20usize, 10usize),
        (50, 25),
        (100, 50),
        (200, 100),
        (500, 250),
    ];
    let max_coeff = 1000i64;
    
    for &(dividend_size, divisor_size) in size_pairs.iter() {
        let dividend_coeffs = generate_random_coeffs(dividend_size, max_coeff);
        let divisor_coeffs = generate_random_coeffs(divisor_size, max_coeff);
        
        let dividend = ZZPoly::from(dividend_coeffs);
        let divisor = ZZPoly::from(divisor_coeffs);
        
        let bench_name = format!("{}%{}", dividend_size, divisor_size);
        
        group.bench_function(BenchmarkId::new("ref_ref", &bench_name), |b| {
            b.iter(|| black_box(&dividend % &divisor))
        });
        
        group.bench_function(BenchmarkId::new("owned_owned", &bench_name), |b| {
            b.iter_with_setup(
                || (dividend.clone(), divisor.clone()),
                |(a, b)| black_box(a % b)
            )
        });
    }
    group.finish();
}

fn bench_rem_assign(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZZPoly RemAssign");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    
    let size_pairs = [
        (50usize, 25usize),
        (100, 50),
        (200, 100),
    ];
    let max_coeff = 1000i64;
    
    for &(dividend_size, divisor_size) in size_pairs.iter() {
        let dividend_coeffs = generate_random_coeffs(dividend_size, max_coeff);
        let divisor_coeffs = generate_random_coeffs(divisor_size, max_coeff);
        
        let dividend = ZZPoly::from(dividend_coeffs);
        let divisor = ZZPoly::from(divisor_coeffs);
        
        let bench_name = format!("{}%={}", dividend_size, divisor_size);
        
        group.bench_function(BenchmarkId::new("assign_ref", &bench_name), |b| {
            b.iter_with_setup(
                || dividend.clone(),
                |mut a| {
                    a %= &divisor;
                    black_box(a)
                }
            )
        });
    }
    group.finish();
}

criterion_group! {
    name = rem_benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = 
        bench_rem_different_sizes,
        bench_rem_assign
}

criterion_main!(rem_benches);