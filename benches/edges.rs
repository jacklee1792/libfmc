use criterion::{Criterion, criterion_group, criterion_main};
use libfmc::*;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("edges compose", |b| {
        let alg = Alg::try_from(
            "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
        )
        .unwrap();
        let a = Edges::default() + alg;
        b.iter(|| black_box(a).compose(black_box(a)))
    });

    c.bench_function("edges inverse", |b| {
        let alg = Alg::try_from(
            "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
        )
        .unwrap();
        let a = Edges::default() + alg;
        b.iter(|| black_box(a).inverse())
    });

    c.bench_function("Corners::compose", |b| {
        let alg = Alg::try_from(
            "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
        )
        .unwrap();
        let a = Corners::default() + alg;
        b.iter(|| black_box(a).compose(black_box(a)))
    });

    c.bench_function("Corners::cofb", |b| {
        let alg = Alg::try_from(
            "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
        )
        .unwrap();
        let a = Corners::default() + alg;
        b.iter(|| black_box(a).cofb(Corner::UFL))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
