use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use advent_of_code_2023_rust::days;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("2023");
    for day in days() {
        group.bench_with_input(
            BenchmarkId::new(day.0, day.1),
            &day.2,
            |b, &input| b.iter(|| day.3(&input)));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);