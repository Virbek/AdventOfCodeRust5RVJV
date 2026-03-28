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

    /*c.bench_function("d5_v1_part1", |b| {
        b.iter(|| d5::v1::solve_part1(black_box(input)))
    });

    c.bench_function("d5_v2_part1", |b| {
        b.iter(|| d5::v2::solve_part1(black_box(input)))
    });

    c.bench_function("d5_v1_part2", |b| {
        b.iter(|| d5::v1::solve_part2(black_box(input)))
    });

    c.bench_function("d5_v2_part2", |b| {
        b.iter(|| d5::v2::solve_part2(black_box(input)))
    });*/
}

criterion_group!(benches, bench_d5);
criterion_main!(benches);

/*
d5_v1_part1             time:   [156.18 µs 157.00 µs 157.84 µs]
                        change: [−8.4319% −7.0860% −5.8676%] (p = 0.00 < 0.05)
d5_v2_part1             time:   [94.725 µs 94.959 µs 95.211 µs]
                        change: [−70.392% −70.060% −69.561%] (p = 0.00 < 0.05)

d5_v1_part2             time:   [14.349 µs 14.366 µs 14.383 µs]
                        change: [−1.7289% −0.5935% +0.1008%] (p = 0.28 > 0.05)

d5_v2_part2             time:   [8.9248 µs 8.9499 µs 8.9797 µs]
                        change: [−2.4884% −1.1460% +0.3739%] (p = 0.12 > 0.05)


 */