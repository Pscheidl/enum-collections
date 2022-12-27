mod enumerated;
use std::{
    alloc::{alloc_zeroed, Layout},
    marker::PhantomData,
    slice::from_raw_parts_mut,
};

pub use enumerated::Enumerated;

///
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
#[allow(dead_code)]
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
    pub fn new() -> Self {
        Self {
            values: unsafe {
                let raw_memory = alloc_zeroed(Layout::array::<Option<V>>(K::len()).unwrap());
                from_raw_parts_mut(raw_memory as *mut Option<V>, K::len())
            },
            _key_phantom_data: PhantomData {},
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.values[key.position()].as_ref()
    }

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

    #[test]
    fn test() {
        use crate::{enummap, EnumMap, Enumerated};
        #[enummap]
        enum Letter {
            A,
            B,
        }

        let mut map: EnumMap<Letter, u8> = EnumMap::new();
        map.insert(Letter::A, 42);
        assert_eq!(Some(&42u8), map.get(Letter::A))
    }
}
