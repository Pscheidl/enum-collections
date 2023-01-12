mod index;
use std::marker::PhantomData;

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
///
/// Using get and insert functions.
/// ```
/// use enum_collections::{EnumTable, Enumerated};
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumTable<Letter, u8> = EnumTable::new();
/// map.insert(Letter::A, 42);
/// assert_eq!(&42u8, map.get(Letter::A));
/// assert_eq!(&u8::default(), map.get(Letter::B));
/// ```
pub struct EnumTable<K, V>
where
    K: Enumerated,
    V: Default,
{
    values: Box<[V]>,
    _key_phantom_data: PhantomData<K>,
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
            values: (0..K::len())
                .map(|_| V::default())
                .collect::<Vec<V>>()
                .into(),
            _key_phantom_data: PhantomData {},
        }
    }

    /// Obtain a value for given `key`, always returning a value `V`,
    /// as the EnumTable is pre-initialized with defaults.
    ///
    /// ### Args
    /// - `key` - Instance of `K`, used to look up the corresponding value.
    #[inline]
    pub fn get(&self, key: K) -> &V {
        &self.values[key.position()]
    }

    /// Stores given `value` under the provided `key`. Overrides any existing corresponding value.
    ///
    /// ### Args
    /// - `key` - The instance of `K` the value inserted can be looked up for.
    /// - `values` - Value to bind to `K`.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        self.values[key.position()] = value
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

#[cfg(test)]
mod tests {
    use crate::Enumerated;

    use super::EnumTable;

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
        for index in 0..Letter::len() {
            assert_eq!(Value::default(), enum_table.values[index]);
        }
    }

    #[test]
    fn inserts() {
        let mut enum_table = EnumTable::<Letter, Value>::new();
        let inserted_value = Value::new("Hello".to_string());
        enum_table.insert(Letter::A, inserted_value.clone());
        assert_eq!(&inserted_value, enum_table.get(Letter::A));
        assert_eq!(&Value::default(), enum_table.get(Letter::B));
    }

    #[test]
    fn reset() {
        let mut enum_table = EnumTable::<Letter, Value>::new();
        let inserted_value = Value::new("Hello".to_string());
        enum_table.insert(Letter::A, inserted_value.clone());
        assert_eq!(&inserted_value, enum_table.get(Letter::A));
        enum_table.reset(Letter::A);
        assert_eq!(&Value::default(), enum_table.get(Letter::A));
        assert_eq!(&Value::default(), enum_table.get(Letter::B));
    }
}
