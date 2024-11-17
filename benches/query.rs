use criterion::{criterion_group, criterion_main, Criterion};
use hashbrown::HashMap as HashBrownMap;
use rustc_hash::FxHashMap;
use std::collections::HashMap;

fn bench_hashmaps(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMaps Insertion");

    group.bench_function("std::HashMap", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..10_000 {
                map.insert(i, i);
            }
            map
        })
    });

    group.bench_function("FxHashMap", |b| {
        b.iter(|| {
            let mut map = FxHashMap::default();
            for i in 0..10_000 {
                map.insert(i, i);
            }
            map
        })
    });

    group.bench_function("HashBrownMap", |b| {
        b.iter(|| {
            let mut map = HashBrownMap::new();
            for i in 0..10_000 {
                map.insert(i, i);
            }
            map
        })
    });

    group.finish();
}

criterion_group!(benches, bench_hashmaps);
criterion_main!(benches);
