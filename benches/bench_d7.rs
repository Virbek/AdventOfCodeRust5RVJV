/* Hyperfine 

ver1 first half:
    Time (mean ± σ):      19.7 ms ±   0.9 ms    [User: 6.3 ms, System: 7.6 ms]
    Range (min … max):    17.9 ms …  22.5 ms    97 runs */
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d7;

fn bench_d7(c: &mut Criterion) {
    let input = d7::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d7_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d7::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d7::v2::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d7_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d7::v1::solve_part2(black_box(input))))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| black_box(d7::v2::solve_part2(black_box(input))))
    });

    group_p2.finish();

}

criterion_group!(benches, bench_d7);
criterion_main!(benches);

/*
    d7_v1_part1             time:   [165.89 µs 169.02 µs 172.43 µs]
    d7_v2_part1             time:   [76.194 µs 77.833 µs 79.511 µs]

    d7_v1_part2             time:   [162.70 µs 165.19 µs 167.78 µs]
    d7_v2_part2             time:   [124.40 µs 126.52 µs 128.64 µs]
 */