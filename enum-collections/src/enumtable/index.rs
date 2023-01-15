use std::ops::{Index, IndexMut};

use crate::{EnumTable, Enumerated};

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

#[cfg(test)]
mod tests {
    use crate::enumtable::tests::Letter;
    use crate::enumtable::tests::Value;

    use super::EnumTable;

    #[test]
    fn get_insert_index_trait() {
        let mut enum_table = EnumTable::<Letter, Value>::new();
        let inserted_value = Value::new("Hello".to_string());
        enum_table[Letter::A] = inserted_value.clone();
        assert_eq!(&inserted_value, &enum_table[Letter::A]);
        assert_eq!(&Value::default(), &enum_table[Letter::B]);
    }
}
