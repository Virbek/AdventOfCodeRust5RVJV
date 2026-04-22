/* Hyperfine 

ver1 first half :
	Time (mean ± σ):      18.6 ms ±   1.2 ms    [User: 5.9 ms, System: 7.4 ms]
  	Range (min … max):    17.3 ms …  28.4 ms    103 runs

	Second half :
	Time (mean ± σ):      19.4 ms ±   1.1 ms    [User: 5.8 ms, System: 8.0 ms]
  	Range (min … max):    18.1 ms …  27.7 ms    101 runs */

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d5;

fn bench_d5(c: &mut Criterion) {
    let input = d5::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d5_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d5::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d5::v2::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v3", |b| {
        b.iter(|| black_box(d5::v3::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d5_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d5::v1::solve_part2(black_box(input))))
    });

    group_p2.bench_function("v2", |b| {
        b.iter(|| black_box(d5::v2::solve_part2(black_box(input))))
    });

    group_p2.bench_function("v3", |b| {
        b.iter(|| black_box(d5::v3::solve_part2(black_box(input))))
    });

    group_p2.finish();

}

criterion_group!(benches, bench_d5);
criterion_main!(benches);

// d5_part1/v1             time:   [381.57 µs 385.49 µs 389.74 µs]
//                         change: [−20.456% −18.442% −16.426%] (p = 0.00 < 0.05)
// d5_part1/v2             time:   [327.33 µs 332.17 µs 337.46 µs]
//                         change: [−23.329% −21.205% −19.077%] (p = 0.00 < 0.05)
// d5_part1/v3             time:   [68.697 µs 69.126 µs 69.577 µs]
//                         change: [−18.250% −15.568% −13.017%] (p = 0.00 < 0.05)

// d5_part2/v1             time:   [24.007 µs 24.184 µs 24.367 µs]
//                         change: [−22.850% −20.762% −18.823%] (p = 0.00 < 0.05)
// d5_part2/v2             time:   [21.296 µs 23.195 µs 25.625 µs]
//                         change: [−11.584% −6.1330% +0.2170%] (p = 0.05 < 0.05)
// d5_part2/v3             time:   [15.848 µs 15.964 µs 16.084 µs]

