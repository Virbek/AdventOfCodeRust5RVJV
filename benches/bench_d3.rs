/* Hyperfine 

Ver 1 : first half

Time (mean ± σ):      19.6 ms ±   0.8 ms    [User: 5.8 ms, System: 6.0 ms]
  Range (min … max):    17.9 ms …  25.1 ms    102 runs
	with second-half:
Time (mean ± σ):      19.0 ms ±   0.7 ms    [User: 8.0 ms, System: 6.0 ms]
  Range (min … max):    17.7 ms …  22.7 ms    104 runs */

// Criterion

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::{d2, d3};

fn bench_d3(c: &mut Criterion) {
    let input = d3::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d3_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| d3::v1::solve_part1(black_box(input)))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| d3::v2::solve_part1(black_box(input)))
    });

    group_p1.bench_function("v3", |b| {
        b.iter(|| d3::v3::solve_part1(black_box(input)))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d3_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| d3::v1::solve_part2(black_box(input)))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| d3::v2::solve_part2(black_box(input)))
    });

    group_p2.finish();
}

criterion_group!(benches, bench_d3);
criterion_main!(benches);

/*

PC Fixe
Partie 1
d3_v1_part1             time:   [461.17 µs 462.99 µs 464.99 µs]
d3_v2_part1             time:   [260.97 µs 261.59 µs 262.30 µs] => 2x plus perf

Partie 2
d3_v1_part2             time:   [481.06 µs 488.25 µs 498.09 µs]
d3_v2_part2             time:   [84.038 µs 84.252 µs 84.508 µs] => 5x plus perf

PC Portable
d3_part1/v1             time:   [1.2469 ms 1.2545 ms 1.2632 ms]
d3_part1/v2             time:   [666.20 µs 669.17 µs 672.37 µs]

d3_part2/v1             time:   [1.2946 ms 1.3111 ms 1.3334 ms]
d3_part2/v2             time:   [155.42 µs 156.19 µs 157.07 µs]
*/