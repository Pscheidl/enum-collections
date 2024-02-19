# Enum Collections for Rust
[![Rust](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml/badge.svg)](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/enum-collections)](https://crates.io/crates/enum-collections)
[![docs.rs](https://img.shields.io/docsrs/enum-collections)](https://docs.rs/enum-collections/latest/enum_collections/)

[Contribution guide](CONTRIBUTING.md) | [Apache v2 license](LICENSE)

Enum-centric data structures for Rust. 

## Usage

### EnumMap

EnumMap is a special case of a Hash Map, with better **computational complexity** guarantees and overall **performance**. Can differentiante between a missing (`Option::None`)
and set (`Option::Some`) value.

Abstracts away the need to handle [Option] on insert/remove operations.
It marginally is faster to initialize than `EnumTable`, because `Default` value needn't be cloned for each field.

Using `get` and `insert` functions.

```rust
 use enum_collections::{EnumMap, Enumerated};
 #[derive(Enumerated)]
 enum Letter {
     A,
     B,
 }
 assert_eq!(Letter::VARIANTS.len(), 2); // VARIANTS provided by this crate
 
 let mut map: EnumMap<Letter, u8> = EnumMap::new();
 map.insert(Letter::A, 42);
 assert_eq!(Some(&42u8), map.get(Letter::A));
 map.remove(Letter::A);
 assert_eq!(None, map.get(Letter::A));
```

Using `Index` and `IndexMut` syntactic sugar.
 ```rust
 use enum_collections::{EnumMap, Enumerated};
 #[derive(Enumerated)]
 enum Letter {
     A,
     B,
 }

 let mut map: EnumMap<Letter, u8> = EnumMap::new();
 map[Letter::A] = Some(42);
 assert_eq!(Some(42u8), map[Letter::A]);
 assert_eq!(Some(&42u8), map[Letter::A].as_ref());
 ```

### EnumTable

EnumTable is a special case of a Hash Map, with better **computational complexity** guarantees and overall **performance**. Initialized with default values, can NOT differentiate between missing values
and values actually set.

Using Index and IndexMut syntactic sugar.

```rust
 use enum_collections::{EnumTable, Enumerated};
 #[derive(Enumerated)]
 enum Letter {
     A,
     B,
 }
 assert_eq!(Letter::VARIANTS.len(), 2); // VARIANTS provided by this crate

 let mut map: EnumTable<Letter, u8> = EnumTable::new();
 map[Letter::A] = 42;
 assert_eq!(42u8, map[Letter::A]);
 assert_eq!(u8::default(), map[Letter::B]);
```

## Features

Portions of functionality are feature-flagged, but enabled by default. This is to allow turning this functionality off when not needed, e.g. `Debug` and `Eq` implementations.

See [docs.rs](https://docs.rs/crate/enum-collections/latest/features) for details.

## Benchmarks

There are single-threaded benchmarks for the `get`, `insert` and `remove` operations in [enum-collections/benches](enum-collections/benches/). Invoke `cargo bench` to run them.

### EnumMap
```
NAME                                     lower bound | est | upper bound
EnumMap get                      time:   [635.02 ps 635.52 ps 636.06 ps] est ~22x faster
std::collections::HashMap get    time:   [13.971 ns 13.986 ns 14.002 ns]

EnumMap insert                   time:   [890.74 ps 892.13 ps 893.66 ps] est ~15,57x faster
std::collections::HashMap insert time:   [13.889 ns 13.895 ns 13.901 ns]

EnumMap remove                   time:   [481.07 ps 481.79 ps 482.53 ps] est ~28,55x faster
std::collections::HashMap remove time:   [13.704 ns 13.737 ns 13.771 ns]
```

### EnumTable

```
NAME                                             lower bound | est | upper bound
EnumTable Index get                      time:   [460.22 ps 460.81 ps 461.41 ps] est ~1.113x faster
Crate Enum-Map Index get                 time:   [512.16 ps 512.62 ps 513.13 ps]

EnumTable insert                         time:   [670.83 ps 671.43 ps 672.06 ps] est. ~1,06x faster
Crate Enum-Map insert                    time:   [715.68 ps 716.34 ps 717.04 ps]
```
