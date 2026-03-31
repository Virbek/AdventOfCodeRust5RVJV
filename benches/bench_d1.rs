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

    // Partie 1
    let mut group_p1 = c.benchmark_group("d1_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| d1::v1::solve_part1(black_box(input)))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| d1::v2::solve_part1(black_box(input)))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d1_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| d1::v1::solve_part2(black_box(input)))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| d1::v2::solve_part2(black_box(input)))
    });

    group_p2.bench_function("v3", |b| {
        b.iter(|| d1::v3::solve_part2(black_box(input)))
    });

    group_p2.finish();
}

criterion_group!(benches, bench_d1);
criterion_main!(benches);

/*
Bench 1 - PC fixe
d1_v1_part1             time:   [323.30 µs 323.94 µs 324.62 µs]
d1_v2_part1             time:   [62.163 µs 63.154 µs 64.355 µs] => La V2 a 5 fois plus de perfs

d1_v1_part2             time:   [493.14 µs 493.84 µs 494.60 µs]
d1_v2_part2             time:   [1.6222 ms 1.6264 ms 1.6327 ms] => V2 à revoir

Bench 2 - PC portable avec V3
d1_v1_part2             time:   [1.3457 ms 1.3471 ms 1.3485 ms]
d1_v2_part2             time:   [4.1934 ms 4.1976 ms 4.2021 ms]
d1_v3_part2             time:   [980.07 µs 994.96 µs 1.0103 ms] => V3 un peu plus perf
 */