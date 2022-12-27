# Enum Collections for Rust
[![Rust](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml/badge.svg)](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml)

Enum Map is a special case of a Hash Map, with better **computational complexity** guarantees and overall **performance**.

## Usage

```rust
use enum_collections::{enummap, EnumMap, Enumerated};
#[enum_collections]
enum Letter {
    A,
    B,
}

let mut map: EnumMap<Letter, u8> = EnumMap::new();
map.insert(Letter::A, 42);
assert_eq!(Some(&42u8), map.get(Letter::A))
```

## Benchmarks

There are single-threaded benchmarks for the `get`, `insert` and `remove` operations in [enum-collections/benches](enum-collections/benches/). Invoke `cargo bench` to run them.

```
NAME                                     lower bound | est | upper bound
EnumMap get                      time:   [635.02 ps 635.52 ps 636.06 ps] est ~22x faster
std::collections::HashMap get    time:   [13.971 ns 13.986 ns 14.002 ns]
EnumMap insert                   time:   [947.20 ps 947.83 ps 948.52 ps] est ~14,7x faster
std::collections::HashMap insert time:   [13.938 ns 13.964 ns 13.994 ns]
EnumMap remove                   time:   [481.07 ps 481.79 ps 482.53 ps] est ~28,55x faster
std::collections::HashMap remove time:   [13.704 ns 13.737 ns 13.771 ns]

Crate Enum-Map get               time:   [459.92 ps 461.23 ps 462.63 ps] est ~0.72x slower
Crate Enum-Map insert            time:   [713.69 ps 714.95 ps 716.37 ps] est ~0.75x slower
The Enum-Map crate doesn't support `remove` operation, because it can't differentiate between default and missing value.
```
