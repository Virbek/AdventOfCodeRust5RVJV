/* Hyperfine 

Ver 1 :
	Time (mean ± σ):     181.9 ms ±  10.2 ms    [User: 147.9 ms, System: 24.9 ms]
	Range (min … max):   173.3 ms … 212.4 ms    16 runs
with second half : 
	Time (mean ± σ):     665.0 ms ±  12.4 ms    [User: 636.2 ms, System: 19.7 ms]
  	Range (min … max):   648.8 ms … 684.4 ms    10 runs */

// Criterion

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d2;

fn bench_d2(c: &mut Criterion) {
    let input = d2::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d2_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d2::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d2::v2::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d2_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d2::v1::solve_part2(black_box(input))))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| black_box(d2::v2::solve_part2(black_box(input))))
    });

    group_p2.finish();
}

criterion_group!(benches, bench_d2);
criterion_main!(benches);

/*
PC Fixe
Partie 1
d2_v1_part1             time:   [86.238 ms 86.574 ms 86.957 ms]
d2_v2_part1             time:   [52.882 ms 53.044 ms 53.207 ms] => Un peu plus perf

Partie 2
d2_v1_part2             time:   [305.83 ms 306.33 ms 306.88 ms]
d2_v2_part2             time:   [82.223 ms 82.428 ms 82.659 ms] => Bien plus perf

PC portable
d2_part1/v1             time:   [268.32 ms 273.03 ms 277.89 ms]
d2_part1/v2             time:   [152.35 ms 156.59 ms 161.64 ms]

d2_part2/v1             time:   [856.07 ms 858.20 ms 860.32 ms]
d2_part2/v2             time:   [210.62 ms 211.10 ms 211.60 ms]
 */