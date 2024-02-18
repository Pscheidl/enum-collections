use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, Index, IndexMut};

use crate::Enumerated;

/// A key-value table optimized for Enums used as keys. Initialized with `V`'s [Default] values.
///
/// ```
/// use enum_collections::{EnumTable, Enumerated};
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumTable<Letter, u8> = EnumTable::new();
/// map[Letter::A] = 42;
/// assert_eq!(42u8, map[Letter::A]);
/// assert_eq!(u8::default(), map[Letter::B]);
/// ```
pub struct EnumTable<K, V> {
    values: Box<[V]>,
    _key_phantom_data: PhantomData<K>,
}

impl<K, V> EnumTable<K, V>
where
    K: Enumerated,
{
    pub fn from_fn(cb: impl FnMut(&K) -> V) -> Self {
        Self {
            values: K::VARIANTS.iter().map(cb).collect::<Vec<V>>().into(),
            _key_phantom_data: Default::default(),
        }
    }
}

impl<K, V> EnumTable<K, V>
where
    K: Enumerated,
    V: Default,
{
    /// Creates a new [EnumTable], with pre-allocated space for all keys of the enum `K`. With the underlying array righsized,
    /// no resizing is further required. All values are initialized with `V`'s [Default] value.
    pub fn new() -> Self {
        Self {
            values: K::VARIANTS
                .iter()
                .map(|_| V::default())
                .collect::<Vec<V>>()
                .into(),
            _key_phantom_data: PhantomData {},
        }
    }

    /// Resets value of type `V` corresponding to `key` to its default by calling its [Default] trait implementation.
    ///
    /// ### Args
    /// - `key` - The instance of `K` pointing at the slot to reset to default.
    pub fn reset(&mut self, key: K) {
        self.values[key.position()] = V::default();
    }
}

impl<K, V> Default for EnumTable<K, V>
where
    K: Enumerated,
    V: Default,
{
    /// Constructs a new instance, capable of holding all values of key `K` without further resizing.
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Index<K> for EnumTable<K, V>
where
    K: Enumerated,
    V: Default,
{
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        &self.values[key.position()]
    }
}

impl<K, V> IndexMut<K> for EnumTable<K, V>
where
    K: Enumerated,
    V: Default,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.values[key.position()]
    }
}

impl<F, K, V> From<F> for EnumTable<K, V>
where
    K: Enumerated,
    F: FnMut(&K) -> V,
{
    fn from(cb: F) -> Self {
        Self::from_fn(cb)
    }
}

impl<K, V> Debug for EnumTable<K, V>
where
    K: Enumerated + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Values are stored in the EnumTable's backing array in the same order as enum's variants
        f.debug_map()
            .entries(
                K::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| (variant, &self.values[index])),
            )
            .finish()
    }
}

impl<K, V> PartialEq<Self> for EnumTable<K, V>
where
    K: Enumerated,
    V: Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values.deref().eq(other.values.deref())
    }
}

impl<K, V> Eq for EnumTable<K, V>
where
    K: Enumerated,
    V: Default + Eq,
{
}

#[cfg(test)]
mod tests {
    use super::EnumTable;
    use crate::Enumerated;

    /// No Debug derived on purpose, the crate must be usable without [std::fmt::Debug] derived
    /// for the enum.
    #[derive(Enumerated)]
    pub(super) enum Letter {
        A,
        B,
    }

    #[derive(Eq, PartialEq, Debug, Clone)]
    pub(super) struct Value {
        name: String,
    }

    impl Value {
        pub(super) fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Default for Value {
        fn default() -> Self {
            Self {
                name: "Non-empty default".to_owned(),
            }
        }
    }

    #[test]
    fn new_all_default() {
        let enum_table = EnumTable::<Letter, Value>::new();
        for index in 0..Letter::VARIANTS.len() {
            assert_eq!(Value::default(), enum_table.values[index]);
        }
    }

    #[test]
    fn get_insert_index_trait() {
        let mut enum_table = EnumTable::<Letter, Value>::new();
        let inserted_value = Value::new("Hello".to_string());
        enum_table[Letter::A] = inserted_value.clone();
        assert_eq!(&inserted_value, &enum_table[Letter::A]);
        assert_eq!(&Value::default(), &enum_table[Letter::B]);
    }

    /// A dedicated enum with [std::fmt::Debug] derived, to test compilation and usability both
    /// with and without `Debug` implemented.
    #[derive(Enumerated, Debug)]
    pub(super) enum LetterDebugDerived {
        A,
        B,
    }

    #[test]
    fn debug() {
        let mut enum_table = EnumTable::<LetterDebugDerived, i32>::new();
        enum_table[LetterDebugDerived::A] = 42;
        let debug_output = format!("{enum_table:?}");
        let expected_output = "{A: 42, B: 0}";
        assert_eq!(expected_output, debug_output);
    }

    #[test]
    fn eq() {
        let mut first_table = EnumTable::<LetterDebugDerived, i32>::new();
        first_table[LetterDebugDerived::A] = 42;
        let mut second_table = EnumTable::<LetterDebugDerived, i32>::new();
        second_table[LetterDebugDerived::A] = 42;
        assert_eq!(first_table, second_table);
        second_table[LetterDebugDerived::B] = 42;
        debug_assert_ne!(first_table, second_table);
    }
}
