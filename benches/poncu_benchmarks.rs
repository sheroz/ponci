use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn poncu_f1(i:u32) -> u32 {i+1}
fn poncu_f2(i:u32) -> u32 {((i as f64) * 1.33_f64) as u32}

fn poncu_benchmark1(c: &mut Criterion) {
    c.bench_function("poncu_f1", |bencher| {
        bencher.iter(|| poncu_f1(black_box(1)))
    });
    c.bench_function("poncu_f2", |bencher| {
        bencher.iter(|| poncu_f2(black_box(2)))
    });
}

fn poncu_benchmark2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poncu");

    for i in [1, 2, 3].iter() {
        group.bench_with_input(BenchmarkId::new("poncu_f1", i), i, |bencher, i| {
            bencher.iter(|| poncu_f1(*i))
        });
        group.bench_with_input(BenchmarkId::new("poncu_f2", i), i, |bencher, i| {
            bencher.iter(|| poncu_f2(*i))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    poncu_benchmark1,
    poncu_benchmark2
);

criterion_main!(benches);