mod index;
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    marker::PhantomData,
    slice::from_raw_parts_mut,
};

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
/// use enum_collections::{enum_collections, EnumMap, Enumerated};
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

pub struct EnumMap<'a, K, V>
where
    K: Enumerated,
{
    values: &'a mut [Option<V>],
    _key_phantom_data: PhantomData<K>,
}

impl<'a, K, V> EnumMap<'a, K, V>
where
    K: Enumerated,
{
    /// Creates a new [EnumMap], with pre-allocated space for all keys of the enum `K`. With the underlying array righsized,
    /// no resizing is further required.
    pub fn new() -> Self {
        Self {
            values: unsafe {
                let raw_memory = alloc_zeroed(Layout::array::<Option<V>>(K::len()).unwrap());
                from_raw_parts_mut(raw_memory as *mut Option<V>, K::len())
            },
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

impl<'a, K, V> Default for EnumMap<'a, K, V>
where
    K: Enumerated,
{
    /// Constructs a new instance, capable of holding all values of key `K` without further resizing.
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V> Drop for EnumMap<'a, K, V>
where
    K: Enumerated,
{
    /// The underlying memory allocated for values must be deallocated manually, as the destruction of the
    /// fat slice pointer doesn't guarantee it.
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.values.as_ptr() as *mut u8,
                Layout::array::<Option<V>>(K::len()).unwrap(),
            );
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Enumerated;

    use super::EnumMap;

    #[derive(Enumerated)]
    pub(super) enum Letter {
        A,
        B,
    }
    #[test]
    fn new_all_none() {
        let enum_map = EnumMap::<Letter, i32>::new();
        for index in 0..Letter::len() {
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
}
