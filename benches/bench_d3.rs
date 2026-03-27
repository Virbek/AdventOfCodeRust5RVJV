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
use AdventOfCodeRust5RVJV::d3;

fn bench_d2(c: &mut Criterion) {
    let input = d3::INPUT;

    c.bench_function("d3_v1_part1", |b| {
        b.iter(|| d3::v1::solve_part1(black_box(input)))
    });

    c.bench_function("d3_v2_part1", |b| {
        b.iter(|| d3::v2::solve_part1(black_box(input)))
    });
    /*
    c.bench_function("d3_v1_part2", |b| {
        b.iter(|| d3::v1::solve_part2(black_box(input)))
    });

    c.bench_function("d3_v2_part2", |b| {
        b.iter(|| d3::v2::solve_part2(black_box(input)))
    });*/
}

criterion_group!(benches, bench_d2);
criterion_main!(benches);

/*

Partie 1
d3_v1_part1             time:   [461.17 µs 462.99 µs 464.99 µs]
d3_v2_part1             time:   [260.97 µs 261.59 µs 262.30 µs] => V2 2x plus perf

Partie 2

 */