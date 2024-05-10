use std::{collections::HashMap, hash::Hash};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enum_collections::{EnumMap, Enumerated};

#[derive(Enumerated, Eq, PartialEq, Hash)] // Enum derived to benchmark against the `enum-map` crate
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
    let mut enum_map: EnumMap<Letter, i32, { Letter::SIZE }> = EnumMap::new_default();
    enum_map[Letter::A] = 1;
    criterion.bench_function("EnumMap get", |bencher| {
        bencher.iter(|| enum_map[Letter::A]) // Tested without a black box, expected to be optimized in real-world usage
    });
}

fn enummap_insert(criterion: &mut Criterion) {
    let mut enum_map: EnumMap<Letter, i32, { Letter::SIZE }> = EnumMap::new_default();
    enum_map[Letter::A] = 1;
    criterion.bench_function("EnumMap insert", |bencher| {
        bencher.iter(|| enum_map[Letter::A] = black_box(42))
    });
}

fn enummap_new_default(criterion: &mut Criterion) {
    criterion.bench_function("EnumMap new: default", |bencher| {
        bencher.iter(|| EnumMap::<Letter, i32, { Letter::SIZE }>::new_default())
    });
}

fn enummap_new_option(criterion: &mut Criterion) {
    criterion.bench_function("EnumMap new: Option::None", |bencher| {
        bencher.iter(|| EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option())
    });
}

fn enummap_new(criterion: &mut Criterion) {
    criterion.bench_function("EnumMap new: provider fn", |bencher| {
        bencher.iter(|| (EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42))) // Tested without a black box, expected to be optimized in real-world usage
    });
}

fn enummap_new_inspect(criterion: &mut Criterion) {
    criterion.bench_function("EnumMap new: inspecting provider fn", |bencher| {
        bencher.iter(|| {
            EnumMap::<Letter, i32, { Letter::SIZE }>::new_inspect(|variant| match variant {
                Letter::A => 1,
                _ => 42,
            })
        })
    });
}

fn std_hashmap_get(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, i32> = HashMap::new();
    hashmap.insert(Letter::A, 1);
    criterion.bench_function("std::collections::HashMap get", |bencher| {
        bencher.iter(|| hashmap.get(&Letter::A))
    });
}

fn std_hashmap_insert(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, i32> = HashMap::new();
    criterion.bench_function("std::collections::HashMap insert", |bencher| {
        bencher.iter(|| hashmap.insert(Letter::A, 1))
    });
}

criterion_group!(
    benches,
    enummap_get,
    enummap_insert,
    enummap_new_default,
    enummap_new_option,
    enummap_new,
    enummap_new_inspect,
    std_hashmap_get,
    std_hashmap_insert,
);
criterion_main!(benches);
