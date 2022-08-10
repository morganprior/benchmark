use benching::partition;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn vector_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("part_vector_length");
    let diff = 50;
    let l = 0;
    let mut v: Vec<i32> = (0..1000)
        .map(|_| rand::thread_rng().gen_range(0..=1000))
        .collect();
    let lengths: Vec<usize> = (100..1000).step_by(100).collect();

    let bench_name = format!("vector_length-{}-{}", l, l + diff);

    for length in lengths.into_iter() {
        v.shuffle(&mut thread_rng());

        group.bench_with_input(
            BenchmarkId::new(&bench_name, length),
            &length,
            |b, length| {
                b.iter(|| partition(&mut v[0..*length], l, l + diff));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, vector_length);
criterion_main!(benches);
