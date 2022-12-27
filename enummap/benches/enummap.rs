use std::{collections::HashMap, hash::Hash};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enum_map::{enummap, EnumMap, Enumerated};

#[derive(Eq, PartialEq, Hash)]
#[enummap]
#[allow(dead_code)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn enummap_get(criterion: &mut Criterion) {
    let mut enum_map: EnumMap<Letter, u8> = EnumMap::new();
    enum_map.insert(Letter::A, 1);
    criterion.bench_function("EnumMap get", |bencher| {
        bencher.iter(|| enum_map.get(black_box(Letter::A)))
    });
}

fn std_hashmap_get(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, u8> = HashMap::new();
    hashmap.insert(Letter::A, 1);
    criterion.bench_function("std::collections::HashMap get", |bencher| {
        bencher.iter(|| hashmap.get(black_box(&Letter::A)))
    });
}

criterion_group!(benches, enummap_get, std_hashmap_get);
criterion_main!(benches);
