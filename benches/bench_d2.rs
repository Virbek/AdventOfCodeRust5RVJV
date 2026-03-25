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

    /*c.bench_function("d2_v1_part1", |b| {
        b.iter(|| d2::v1::solve_part1(black_box(input)))
    });*/

    c.bench_function("d2_v2_part1", |b| {
        b.iter(|| d2::v2::solve_part1(black_box(input)))
    });
    /*
    c.bench_function("d1_v1_part2", |b| {
        b.iter(|| d1::v1::solve_part2(black_box(input)))
    });

    c.bench_function("d1_v2_part2", |b| {
        b.iter(|| d1::v2::solve_part2(black_box(input)))
    });*/
}

criterion_group!(benches, bench_d2);
criterion_main!(benches);

// d2_v2_part1             time:   [52.882 ms 53.044 ms 53.207 ms]