/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d10;

fn bench_d10(c: &mut Criterion) {
    let input = d10::INPUT;

    c.bench_function("d10_v1_part1", |b| {
        b.iter(|| black_box(d10::v1::solve_part1(black_box(input))))
    });



    
}

criterion_group!(benches, bench_d10);
criterion_main!(benches);


// d10_v1_part1            time:   [8.1342 ms 8.3614 ms 8.5974 ms]
// Found 2 outliers among 100 measurements (2.00%)



