/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d8;

fn bench_d8(c: &mut Criterion) {
    let input = d8::INPUT;

    c.bench_function("d8_v1_part1", |b| {
        b.iter(|| d8::v1::solve_part1(black_box(input)))
    });

    // c.bench_function("d8_v2_part1", |b| {
    //     b.iter(|| d8::v2::solve_part1(black_box(input)))
    // });

    c.bench_function("d8_v1_part2", |b| {
        b.iter(|| d8::v1::solve_part2(black_box(input)))
    });

    // c.bench_function("d8_v2_part2", |b| {
    //     b.iter(|| d8::v2::solve_part2(black_box(input)))
    // });
}

criterion_group!(benches, bench_d8);
criterion_main!(benches);

/* M4 Pro 24Go RAM
    d8_v1_part1             time:   [14.522 ms 14.541 ms 14.561 ms]
*/


/* M4 Pro 24Go RAM
    d8_v1_part2             time:   [16.320 ms 16.355 ms 16.398 ms]
*/

/*
    d8_v1_part1             time:   [28.822 ms 29.100 ms 29.388 ms]

    d8_v1_part2             time:   [31.494 ms 31.790 ms 32.096 ms]
 */