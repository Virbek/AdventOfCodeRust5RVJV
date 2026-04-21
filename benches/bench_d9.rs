/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d9;

fn bench_d9(c: &mut Criterion) {
    let input = d9::INPUT;

    c.bench_function("d9_v1_part1", |b| {
        b.iter(|| black_box(d9::v1::solve_part1(black_box(input))))
    });


    c.bench_function("d9_v1_part2", |b| {
        b.iter(|| black_box(d9::v1::solve_part2(black_box(input))))
    });

    
}

criterion_group!(benches, bench_d9);
criterion_main!(benches);


// d9_v1_part1             time:   [1.1480 ms 1.1705 ms 1.1949 ms]
// Found 7 outliers among 100 measurements (7.00%)


// d9_v1_part2             time:   [24.819 ms 25.535 ms 26.261 ms]
// Found 1 outliers among 100 measurements (1.00%)

