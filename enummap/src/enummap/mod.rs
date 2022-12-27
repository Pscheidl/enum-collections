mod enumerated;
use std::marker::PhantomData;

pub use enumerated::Enumerated;

#[allow(dead_code)]
struct EnumMap<'a, K, V>
where
    K: Enumerated,
{
    values: &'a mut [V],
    _key_phantom_data: PhantomData<K>,
}

impl<'a, K, V> EnumMap<'a, K, V>
where
    K: Enumerated,
{
    #[allow(dead_code)]
    fn get(&self, key: K) -> Option<&V> {
        self.values.get(key.position())
    }
}
