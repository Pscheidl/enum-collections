# Enum Collections for Rust
[![Rust](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml/badge.svg)](https://github.com/Pscheidl/enum-map/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/enum-collections)](https://crates.io/crates/enum-collections)
[![docs.rs](https://img.shields.io/docsrs/enum-collections)](https://docs.rs/enum-collections/latest/enum_collections/)

[Contribution guide](CONTRIBUTING.md) | [Apache v2 license](LICENSE)


A map of enum variants to values. EnumMap is a fixed-size map, where each variant of the enum
is mapped to a value. This implementation EnumMap is a a zero-cost abstraction over an array (const-sized), where the index of the array corresponds to the position of the variant in the enum.

Because it is a thin wrapper over an array, it is stack-allocated by default. Simply [std::boxed::Box]ing it will move it to the heap, at the caller's discretion.

- Indexed by enum variants.
- IndexMut by enum variants.
- Debug if the enum is Debug.
- PartialEq if the value is PartialEq. Same for Eq.

Debug and Eq are optional features. They are enabled by default.

## Usage

```rust
use enum_collections::{EnumMap, Enumerated};

#[derive(Enumerated)]
pub enum Letter {
   A,
   B,
}

// Indexing and mutation
let mut enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
assert_eq!(0, enum_map[Letter::A]);
enum_map[Letter::A] = 42;
assert_eq!(42, enum_map[Letter::A]);

// Constructor with default values
let enum_map_default = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
assert_eq!(0, enum_map_default[Letter::A]);
assert_eq!(0, enum_map_default[Letter::B]);

// Convenience constructor for optional values
let mut enum_map_option = EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option();
assert_eq!(None, enum_map_option[Letter::A]);
assert_eq!(None, enum_map_option[Letter::B]);
enum_map_option[Letter::A] = Some(42);
assert_eq!(Some(42), enum_map_option[Letter::A]);

// Constructor with custom initialization
#[derive(PartialEq, Eq, Debug)]
struct Custom;
let enum_map = EnumMap::<Letter, Custom, { Letter::SIZE }>::new(|| Custom);
assert_eq!(Custom, enum_map[Letter::A]);
assert_eq!(Custom, enum_map[Letter::B]);

// Custom initialization function with enum variant (key) inspection
let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_inspect(|letter| {
   match letter {
      Letter::A => 42,
      Letter::B => 24,
   }
});
assert_eq!(42, enum_map[Letter::A]);
assert_eq!(24, enum_map[Letter::B]);

// Debug
#[derive(Enumerated, Debug)]
pub enum LetterDebugDerived {
   A,
   B,
}
let enum_map_debug =
    EnumMap::<LetterDebugDerived, i32, { LetterDebugDerived::SIZE }>::new(|| 42);
assert_eq!("{A: 42, B: 42}", format!("{:?}", enum_map_debug));

```


Iterate over enum variants


```rust
#[derive(Enumerated, Debug)]
pub enum Letter {
   A,
   B,
}

Letter::VARIANTS
    .iter()
    .for_each(|letter| println!("{:?}", letter));
```


## Features

Portions of functionality are feature-flagged, but enabled by default. This is to allow turning this functionality off when not needed, e.g. `Debug` and `Eq` implementations.
See [docs.rs](https://docs.rs/crate/enum-collections/latest/features) for details.

## Benchmarks

Invoke `cargo bench` to run benchmarks. While `EnumMap` operates in pico-seconds, `std::collections::HashMap` in > 10 nanoseconds.

<details>
<summary>Benchmark results</summary>

```
EnumMap get             time:   [221.09 ps 221.59 ps 222.21 ps]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

EnumMap insert          time:   [230.05 ps 233.38 ps 236.25 ps]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

EnumMap new: default    time:   [852.31 ps 853.28 ps 854.37 ps]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

EnumMap new: Option::None
                        time:   [1.7100 ns 1.7110 ns 1.7120 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

EnumMap new: provider fn
                        time:   [791.17 ps 792.38 ps 793.65 ps]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

EnumMap new: inspecting provider fn
                        time:   [775.03 ps 776.84 ps 778.92 ps]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

std::collections::HashMap get
                        time:   [13.433 ns 13.484 ns 13.543 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

std::collections::HashMap insert
                        time:   [14.094 ns 14.107 ns 14.121 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

```

</details>
