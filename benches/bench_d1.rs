/* Hyperfine 

Day 1 Partie 1

ver 1 (Evann) :
Time (mean ± σ):       6.5 ms ±   1.2 ms    [User: 1.8 ms, System: 3.4 ms]
Range (min … max):     4.9 ms …  11.6 ms    100 runs

ver 2 (Côme) :
Time (mean ± σ):       5.7 ms ±   0.8 ms    [User: 2.4 ms, System: 4.8 ms]
Range (min … max):     4.3 ms …  10.5 ms    100 runs

Day 1 Partie 2

ver 1 (Evann) :
Time (mean ± σ):       6.4 ms ±   1.1 ms    [User: 2.5 ms, System: 2.6 ms]
Range (min … max):     5.0 ms …  10.1 ms    100 runs

ver 2 (Côme)

Time (mean ± σ):       7.5 ms ±   0.9 ms    [User: 4.8 ms, System: 3.8 ms]
Range (min … max):     6.4 ms …  13.0 ms    100 runs */

// Criterion

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d1;

fn bench_d1(c: &mut Criterion) {
    let input = d1::INPUT;

    c.bench_function("d1_v1_part1", |b| {
        b.iter(|| d1::v1::solve_part1(black_box(input)))
    });

    c.bench_function("d1_v2_part1", |b| {
        b.iter(|| d1::v2::solve_part1(black_box(input)))
    });
}

criterion_group!(benches, bench_d1);
criterion_main!(benches);