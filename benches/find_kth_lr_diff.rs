use benching::find_kth;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn lr_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_kth_lr_diff");
    let l = 0;
    let k = 20; 
    let mut v: Vec<i32> = (0..1000)
        .map(|_| rand::thread_rng().gen_range(0..=1000))
        .collect();
    let diffs: Vec<usize> = (21..1000).step_by(50).collect();

    for &diff in diffs.iter() {
        group.bench_with_input(BenchmarkId::new("lr_diff", diff), &diff, |b, &diff| {
            b.iter(|| find_kth(&mut v, l, l + diff, k));
        });
    }
    group.finish();
}

criterion_group!(benches, lr_diff);
criterion_main!(benches);
