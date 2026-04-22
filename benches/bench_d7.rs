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

    group_p1.bench_function("v3", |b| {
        b.iter(|| black_box(d7::v3::solve_part1(black_box(input))))
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
    
    group_p2.bench_function("v3", |b| {
        b.iter(|| black_box(d7::v3::solve_part2(black_box(input))))
    });

    group_p2.finish();
    

}

criterion_group!(benches, bench_d7);
criterion_main!(benches);

// d7_part1/v1             time:   [176.12 µs 179.03 µs 182.26 µs]
//                         change: [−25.228% −22.171% −19.259%] (p = 0.00 < 0.05)
// d7_part1/v2             time:   [118.57 µs 120.69 µs 122.76 µs]
//                         change: [−26.117% −23.041% −20.220%] (p = 0.00 < 0.05)
// d7_part1/v3             time:   [85.237 µs 86.482 µs 87.718 µs]
//                         change: [−32.499% −30.103% −27.880%] (p = 0.00 < 0.05)

// d7_part2/v1             time:   [307.90 µs 312.82 µs 319.28 µs]
//                         change: [−19.234% −16.810% −14.287%] (p = 0.00 < 0.05)
// d7_part2/v2             time:   [210.14 µs 213.49 µs 217.44 µs]
//                         change: [−30.518% −28.202% −25.762%] (p = 0.00 < 0.05)
// d7_part2/v3             time:   [33.063 µs 33.780 µs 34.646 µs]
