use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, Index, IndexMut};

use crate::Enumerated;

/// A key-value map optimized for Enums used as keys.
///
/// Abstracts away the need to handle [Option] on insert/remove operations.
/// It is faster to initialize than `EnumTable`, because `Default` value needn't be cloned for each field.
///
/// ## Examples
///
/// Using `get` and `insert` functions.
///
/// ```
/// use enum_collections::{EnumMap, Enumerated};
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumMap<Letter, u8> = EnumMap::new();
/// map.insert(Letter::A, 42);
/// assert_eq!(Some(&42u8), map.get(Letter::A));
/// map.remove(Letter::A);
/// assert_eq!(None, map.get(Letter::A));
/// ```
///
/// Using `Index` and `IndexMut` syntactic sugar.
/// ```
/// use enum_collections::{EnumMap, Enumerated};
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumMap<Letter, u8> = EnumMap::new();
/// map[Letter::A] = Some(42);
/// assert_eq!(Some(42u8), map[Letter::A]);
/// assert_eq!(Some(&42u8), map[Letter::A].as_ref());
/// ```

pub struct EnumMap<K, V>
where
    K: Enumerated,
{
    values: Box<[Option<V>]>,
    _key_phantom_data: PhantomData<K>,
}

impl<K, V> EnumMap<K, V>
where
    K: Enumerated,
{
    /// Creates a new [EnumMap], with pre-allocated space for all keys of the enum `K`. With the underlying array righsized,
    /// no resizing is further required.
    pub fn new() -> Self {
        Self {
            values: K::VARIANTS.iter().map(|_| None).collect::<Vec<_>>().into(),
            _key_phantom_data: PhantomData {},
        }
    }

    /// Attemps to obtain a value for given `key`, returning `Some(V)` if found,
    /// or `None` if no value has been inserted for given key yet.
    ///
    /// ### Args
    /// - `key` - Instance of `K`, used to look up the corresponding value.
    #[inline]
    pub fn get(&self, key: K) -> Option<&V> {
        self.values[key.position()].as_ref()
    }

    /// Stores given `value` under the provided `key`. Overrides any existing value previously set.
    ///
    /// ### Args
    /// - `key` - The instance of `K` the value inserted can be looked up for.
    /// - `values` - Value to bind to `K`.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        self.values[key.position()] = Some(value);
    }

    /// Removes value stored under given key. Further `get` operations are going to return `None`.
    #[inline]
    pub fn remove(&mut self, key: K) {
        self.values[key.position()] = None;
    }
}

impl<K, V> Default for EnumMap<K, V>
where
    K: Enumerated,
{
    /// Constructs a new instance, capable of holding all values of key `K` without further resizing.
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Index<K> for EnumMap<K, V>
where
    K: Enumerated,
    V: Default,
{
    type Output = Option<V>;

    fn index(&self, key: K) -> &Self::Output {
        &self.values[key.position()]
    }
}

impl<K, V> IndexMut<K> for EnumMap<K, V>
where
    K: Enumerated,
    V: Default,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.values[key.position()]
    }
}

impl<K, V> Debug for EnumMap<K, V>
where
    K: Enumerated + Debug,
    V: Default + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl<K, V> PartialEq<Self> for EnumMap<K, V>
where
    K: Enumerated,
    V: Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values.deref().eq(other.values.deref())
    }
}

impl<K, V> Eq for EnumMap<K, V>
where
    K: Enumerated,
    V: Default + Eq,
{
}

#[cfg(test)]
mod tests {
    use crate::Enumerated;

    use super::EnumMap;

    /// No Debug derived on purpose, the crate must be usable without [std::fmt::Debug] derived
    /// for the enum.
    #[derive(Enumerated)]
    pub(super) enum Letter {
        A,
        B,
    }

    #[test]
    fn get_insert_index_trait() {
        let mut enum_map = EnumMap::<Letter, i32>::new();
        enum_map[Letter::A] = Some(42);
        assert_eq!(Some(42), enum_map[Letter::A]);
        assert_eq!(Some(&42), enum_map[Letter::A].as_ref());
        assert_eq!(None, enum_map[Letter::B]);
    }

    #[test]
    fn new_all_none() {
        let enum_map = EnumMap::<Letter, i32>::new();
        for index in 0..Letter::VARIANTS.len() {
            assert_eq!(None, enum_map.values[index]);
        }
    }

    #[test]
    fn inserts() {
        let mut enum_map = EnumMap::<Letter, i32>::new();
        enum_map.insert(Letter::A, 42);
        assert_eq!(Some(&42), enum_map.get(Letter::A));
        assert_eq!(None, enum_map.get(Letter::B));
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
        let mut enum_map = EnumMap::<LetterDebugDerived, i32>::new();
        enum_map.insert(LetterDebugDerived::A, 42);
        let debug_output = format!("{enum_map:?}");
        let expected_output = "{A: Some(42), B: None}";
        assert_eq!(expected_output, debug_output);
    }

    #[test]
    fn eq() {
        let mut first_map = EnumMap::<LetterDebugDerived, i32>::new();
        first_map.insert(LetterDebugDerived::A, 42);
        let mut second_map = EnumMap::<LetterDebugDerived, i32>::new();
        second_map.insert(LetterDebugDerived::A, 42);
        assert_eq!(first_map, second_map);
        second_map.insert(LetterDebugDerived::B, 0);
        debug_assert_ne!(first_map, second_map);
    }
}
