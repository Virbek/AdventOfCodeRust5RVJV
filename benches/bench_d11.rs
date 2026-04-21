/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d11;

fn bench_d11(c: &mut Criterion) {
    let input = d11::INPUT;

    c.bench_function("d11_v1_part1", |b| {
        b.iter(|| black_box(d11::v1::solve_part1(black_box(input))))
    });


    c.bench_function("d11_v1_part2", |b| {
        b.iter(|| black_box(d11::v1::solve_part2(black_box(input))))
    });

    
}

criterion_group!(benches, bench_d11);
criterion_main!(benches);


// d11_v1_part1            time:   [517.65 µs 532.24 µs 546.65 µs]
// Found 1 outliers among 100 measurements (1.00%)

// d11_v1_part2            time:   [1.2484 ms 1.2781 ms 1.3110 ms]
// Found 7 outliers among 100 measurements (7.00%)


