use std::ops::{Index, IndexMut};

use crate::{EnumMap, Enumerated};

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

#[cfg(test)]
mod tests {
    use super::EnumMap;
    use crate::enummap::tests::Letter;

    #[test]
    fn get_insert_index_trait() {
        let mut enum_map = EnumMap::<Letter, i32>::new();
        enum_map[Letter::A] = Some(42);
        assert_eq!(Some(42), enum_map[Letter::A]);
        assert_eq!(Some(&42), enum_map[Letter::A].as_ref());
        assert_eq!(None, enum_map[Letter::B]);
    }
}
