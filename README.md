# Enum Collections for Rust
[![Rust](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml/badge.svg)](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml)

Enum Map is a special case of a Hash Map, with better **computational complexity** guarantees and overall **performance**.

## Usage

### EnumMap

EnumMap is a special case of a Hash Map, with better **computational complexity** guarantees and overall **performance**. Can differentiante between a missing (`Option::None`)
and set (`Option::Some`) value.

Using `get` and `insert` functions.

```rust
 use enum_collections::{enum_collections, EnumMap, Enumerated};
 #[enum_collections]
 enum Letter {
     A,
     B,
 }

 let mut map: EnumMap<Letter, u8> = EnumMap::new();
 map.insert(Letter::A, 42);
 assert_eq!(Some(&42u8), map.get(Letter::A));
 map.remove(Letter::A);
 assert_eq!(None, map.get(Letter::A));
```

Using `Index` and `IndexMut` syntactic sugar.
 ```rust
 use enum_collections::{enum_collections, EnumMap, Enumerated};
 #[enum_collections]
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

```rust
 use enum_collections::{enum_collections, EnumTable, Enumerated};
 #[enum_collections]
 enum Letter {
     A,
     B,
 }

 let mut map: EnumTable<Letter, u8> = EnumTable::new();
 map[Letter::A] = 42;
 assert_eq!(42u8, map[Letter::A]);
 assert_eq!(u8::default(), map[Letter::B]);
```

## Benchmarks

There are single-threaded benchmarks for the `get`, `insert` and `remove` operations in [enum-collections/benches](enum-collections/benches/). Invoke `cargo bench` to run them.

### EnumMap
```
NAME                                     lower bound | est | upper bound
EnumMap get                      time:   [635.02 ps 635.52 ps 636.06 ps] est ~22x faster
std::collections::HashMap get    time:   [13.971 ns 13.986 ns 14.002 ns]

EnumMap insert                   time:   [947.20 ps 947.83 ps 948.52 ps] est ~14,7x faster
std::collections::HashMap insert time:   [13.938 ns 13.964 ns 13.994 ns]

EnumMap remove                   time:   [481.07 ps 481.79 ps 482.53 ps] est ~28,55x faster
std::collections::HashMap remove time:   [13.704 ns 13.737 ns 13.771 ns]
```

### EnumTable

```
NAME                                             lower bound | est | upper bound
EnumTable Index get                      time:   [460.22 ps 460.81 ps 461.41 ps] est ~1.113x faster
Crate Enum-Map Index get                 time:   [512.16 ps 512.62 ps 513.13 ps]

EnumTable IndexMut insert                time:   [746.66 ps 747.67 ps 748.79 ps] est ~0.943x speed
Crate Enum-Map insert                    time:   [704.81 ps 705.35 ps 705.92 ps] est ~1.057x faster
```
