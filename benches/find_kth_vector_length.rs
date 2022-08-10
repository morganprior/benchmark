use benching::find_kth;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn vector_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("part_vector_length");
    let l = 0;
    let mut v: Vec<i32> = (0..1000)
        .map(|_| rand::thread_rng().gen_range(0..=1000))
        .collect();
    let lengths: Vec<usize> = (100..1000).step_by(100).collect();

    for length in lengths.into_iter() {
        v.shuffle(&mut thread_rng());

        group.bench_with_input(
            BenchmarkId::new("vector_length", length),
            &length,
            |b, length| {
                b.iter(|| find_kth(&mut v[0..*length], l, l + *length-1, 99));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, vector_length);
criterion_main!(benches);