/* Hyperfine 

ver1 first half:
    
*/
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use AdventOfCodeRust5RVJV::d11;

fn bench_d11(c: &mut Criterion) {
    let input = d11::INPUT;

    // Partie 1
    let mut group_p1 = c.benchmark_group("d11_part1");

    group_p1.bench_function("v1", |b| {
        b.iter(|| black_box(d11::v1::solve_part1(black_box(input))))
    });

    group_p1.bench_function("v2", |b| {
        b.iter(|| black_box(d11::v2::solve_part1(black_box(input))))
    });

    group_p1.finish();

    // Partie 2
    let mut group_p2 = c.benchmark_group("d11_part2");

    group_p2.bench_function("v1", |b| {
        b.iter(|| black_box(d11::v1::solve_part2(black_box(input))))
    });
    group_p2.bench_function("v2", |b| {
        b.iter(|| black_box(d11::v2::solve_part2(black_box(input))))
    });

    group_p2.finish();

    
}

criterion_group!(benches, bench_d11);
criterion_main!(benches);


// d11_part1/v1            time:   [297.67 µs 299.14 µs 300.59 µs]
//                         change: [−47.143% −45.196% −43.207%] (p = 0.00 < 0.05)

// d11_part1/v2            time:   [167.62 µs 168.51 µs 169.35 µs]
//                         change: [−51.531% −49.593% −47.593%] (p = 0.00 < 0.05)


// d11_part2/v1            time:   [731.84 µs 737.16 µs 744.47 µs]
//                         change: [−45.577% −43.971% −42.329%] (p = 0.00 < 0.05)

// d11_part2/v2            time:   [403.79 µs 407.02 µs 410.36 µs]



