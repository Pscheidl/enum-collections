mod enumerated;
use std::{
    alloc::{alloc_zeroed, Layout},
    marker::PhantomData,
    slice::from_raw_parts_mut,
};

pub use enumerated::Enumerated;

/// A key-value map optimized for Enums used as keys.
///
/// ```
/// use enum_map::{enummap, EnumMap, Enumerated};
/// #[enummap]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumMap<Letter, u8> = EnumMap::new();
/// map.insert(Letter::A, 42);
/// assert_eq!(Some(&42u8), map.get(Letter::A))
///```
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
    pub fn get(&self, key: K) -> Option<&V> {
        self.values[key.position()].as_ref()
    }

    /// Stores given `value` under the provided `key`. Overrides any existing value previously set.
    ///
    /// ### Args
    /// - `key` - The instance of `K` the value inserted can be looked up for.
    /// - `values` - Value to bind to `K`.
    pub fn insert(&mut self, key: K, value: V) {
        self.values[key.position()] = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::Enumerated;
    use enummap_macros::enummap;

    use super::EnumMap;

    #[enummap]
    enum Letter {
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
