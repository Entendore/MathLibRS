use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_rs::{Mat4, Vec4};

fn bench_mat4_mul(c: &mut Criterion) {
    let a = Mat4::from_translation([1.0, 2.0, 3.0].into());
    let b = Mat4::from_translation([4.0, 5.0, 6.0].into());

    c.bench_function("mat4 * mat4", |bencher| {
        bencher.iter(|| { let _ = black_box(a) * black_box(b); })
    });

    let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
    c.bench_function("mat4 * vec4 (simd)", |bencher| {
        bencher.iter(|| { let _ = black_box(a).mul_vec4_simd(black_box(v)); })
    });
}

criterion_group!(benches, bench_mat4_mul);
criterion_main!(benches);
