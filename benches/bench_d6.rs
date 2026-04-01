/* Hyperfine 

ver1 first-half: 
    Time (mean ± σ):      19.5 ms ±   1.5 ms    [User: 6.3 ms, System: 7.2 ms]
    Range (min … max):    17.8 ms …  28.3 ms    103 runs
    second-half:
    Time (mean ± σ):      20.5 ms ±   1.0 ms    [User: 6.1 ms, System: 4.5 ms]
    Range (min … max):    18.8 ms …  23.7 ms    92 runs */

// Criterion

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::{d3, d6};

fn bench_d6(c: &mut Criterion) {
    let input = d6::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d6_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d6::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d6::v2::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d6_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d6::v1::solve_part2(black_box(input))))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| black_box(d6::v2::solve_part2(black_box(input))))
    });

    group_p2.finish();
}

criterion_group!(benches, bench_d6);
criterion_main!(benches);

/*
PC Fixe
Partie 1
d6_v1_part1             time:   [276.72 µs 277.66 µs 278.74 µs]
d6_v2_part1             time:   [139.26 µs 139.63 µs 140.00 µs] => 2x plus de perf

Partie 2
d6_v1_part2             time:   [301.06 µs 302.38 µs 303.75 µs]
d6_v2_part2             time:   [104.78 µs 105.26 µs 105.77 µs] => 3x plus perf

PC Portable
d6_part1/v1             time:   [675.61 µs 679.74 µs 684.72 µs]
d6_part1/v2             time:   [351.07 µs 352.14 µs 353.47 µs]

d6_part2/v1             time:   [705.53 µs 706.81 µs 708.14 µs]
d6_part2/v2             time:   [261.93 µs 263.34 µs 265.23 µs]
*/