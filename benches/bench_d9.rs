/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d9;

fn bench_d9(c: &mut Criterion) {
    let input = d9::INPUT;

    let mut group_p1 = c.benchmark_group("d9_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d9::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d9::v2::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d9_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d9::v1::solve_part2(black_box(input))))
    });

    group_p2.finish();

    
}

criterion_group!(benches, bench_d9);
criterion_main!(benches);


// d9_part1/v1             time:   [873.40 µs 887.02 µs 902.15 µs]
// Found 3 outliers among 100 measurements (3.00%)
//   3 (3.00%) high mild
// d9_part1/v2             time:   [228.39 µs 234.05 µs 240.04 µs]
// Found 3 outliers among 100 measurements (3.00%)
//   3 (3.00%) high mild

// d9_part2/v1             time:   [18.721 ms 19.571 ms 20.502 ms]
// Found 5 outliers among 100 measurements (5.00%)


