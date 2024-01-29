use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_tracer_rs::matrix::Matrix4x4;

fn matrix_4x4_inverse(data: &[f64; 16]) {
    Matrix4x4::new(*data).inverse();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Matrix4x4 inverse", |b| {
        b.iter(|| {
            matrix_4x4_inverse(black_box(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
