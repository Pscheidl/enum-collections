extern crate enum_map;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enum_collections::{EnumTable, Enumerated};
use enum_map::Enum;

#[derive(Enum, Enumerated)] // Enum derived to benchmark against the `enum-map` crate
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

fn enumtable_get(criterion: &mut Criterion) {
    let mut enum_map: EnumTable<Letter, u8> = EnumTable::new();
    enum_map.insert(Letter::A, 1);
    criterion.bench_function("EnumTable get", |bencher| {
        bencher.iter(|| enum_map.get(black_box(Letter::A)))
    });
}

fn enumtable_insert(criterion: &mut Criterion) {
    let mut enum_map: EnumTable<Letter, u8> = EnumTable::new();
    criterion.bench_function("EnumTable insert", |bencher| {
        bencher.iter(|| enum_map.insert(black_box(Letter::A), black_box(1)))
    });
}

fn enumtable_index_get(criterion: &mut Criterion) {
    let mut enum_map: EnumTable<Letter, u8> = EnumTable::new();
    enum_map.insert(Letter::A, 1);
    criterion.bench_function("EnumTable Index get", |bencher| {
        bencher.iter(|| enum_map[black_box(Letter::A)])
    });
}

fn enumtable_index_mut_insert(criterion: &mut Criterion) {
    let mut enum_map: EnumTable<Letter, u8> = EnumTable::new();
    criterion.bench_function("EnumTable IndexMut insert", |bencher| {
        bencher.iter(|| enum_map[black_box(Letter::A)] = black_box(1))
    });
}

// The EnumMap from `EnumMap` crate is much closer counterpart to this crate's `EnumTable` than to `EnumMap`,
// as it can't differentiate missing and default values.
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
    enumtable_get,
    enumtable_insert,
    enumtable_index_get,
    enumtable_index_mut_insert,
    enum_map_crate_get,
    enum_map_crate_insert
);
criterion_main!(benches);
