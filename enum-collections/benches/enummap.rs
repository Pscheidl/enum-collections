extern crate enum_map;
use std::{collections::HashMap, hash::Hash};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enum_collections::{enum_collections, EnumMap, Enumerated};
use enum_map::Enum;

#[derive(Eq, PartialEq, Hash, Enum)] // Enum derived to benchmark against the `enum-map` crate
#[enum_collections]
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

fn enummap_insert(criterion: &mut Criterion) {
    let mut enum_map: EnumMap<Letter, u8> = EnumMap::new();
    criterion.bench_function("EnumMap insert", |bencher| {
        bencher.iter(|| enum_map.insert(black_box(Letter::A), black_box(1)))
    });
}

fn enummap_remove(criterion: &mut Criterion) {
    let mut enum_map: EnumMap<Letter, u8> = EnumMap::new();
    criterion.bench_function("EnumMap remove", |bencher| {
        bencher.iter(|| enum_map.remove(black_box(Letter::A)))
    });
}

fn std_hashmap_get(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, u8> = HashMap::new();
    hashmap.insert(Letter::A, 1);
    criterion.bench_function("std::collections::HashMap get", |bencher| {
        bencher.iter(|| hashmap.get(black_box(&Letter::A)))
    });
}

fn std_hashmap_insert(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, u8> = HashMap::new();
    criterion.bench_function("std::collections::HashMap insert", |bencher| {
        bencher.iter(|| hashmap.insert(black_box(Letter::A), black_box(1)))
    });
}

fn std_hashmap_remove(criterion: &mut Criterion) {
    let mut hashmap: HashMap<Letter, u8> = HashMap::new();
    criterion.bench_function("std::collections::HashMap remove", |bencher| {
        bencher.iter(|| hashmap.remove(black_box(&Letter::A)))
    });
}

fn enum_map_crate_get(criterion: &mut Criterion) {
    let mut enum_map: enum_map::EnumMap<Letter, u8> = enum_map::EnumMap::default();
    enum_map[Letter::A] = 1;
    criterion.bench_function("Crate Enum-Map get", |bencher| {
        bencher.iter(|| enum_map[black_box(Letter::A)])
    });
}

fn enum_map_crate_insert(criterion: &mut Criterion) {
    let mut enum_map: enum_map::EnumMap<Letter, u8> = enum_map::EnumMap::default();
    enum_map[Letter::A] = 1;

    criterion.bench_function("Crate Enum-Map insert", |bencher| {
        bencher.iter(|| enum_map[black_box(Letter::A)] = black_box(1))
    });
}

criterion_group!(
    benches,
    enummap_get,
    std_hashmap_get,
    enummap_insert,
    std_hashmap_insert,
    enummap_remove,
    std_hashmap_remove,
    enum_map_crate_get,
    enum_map_crate_insert
);
criterion_main!(benches);
