use serde::{de::Visitor, Deserialize, Serialize};
use std::{any::type_name, marker::PhantomData};

use crate::{EnumMap, Enumerated};

impl<K: Enumerated + Serialize, V: Serialize, const N: usize> Serialize for EnumMap<K, V, N> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.iter_kv())
    }
}

struct EnumMapVisitor<
    'de,
    K: Enumerated + Deserialize<'de> + PartialEq,
    V: Deserialize<'de> + Default,
    const N: usize,
> {
    marker: PhantomData<&'de (K, V)>,
}

impl<
        'de,
        K: Enumerated + Deserialize<'de> + PartialEq,
        V: Deserialize<'de> + Default,
        const N: usize,
    > Visitor<'de> for EnumMapVisitor<'de, K, V, N>
{
    type Value = EnumMap<K, V, N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "EnumMap<{}, {}>",
            type_name::<K>(),
            type_name::<V>()
        )
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut data = std::array::from_fn(|_| V::default());
        while let Some((k, v)) = map.next_entry::<K, V>()? {
            data[k.position()] = v;
        }

        Ok(EnumMap {
            data,
            _key: PhantomData,
        })
    }
}

impl<
        'de,
        K: Enumerated + Deserialize<'de> + PartialEq,
        V: Deserialize<'de> + Default + 'de,
        const N: usize,
    > Deserialize<'de> for EnumMap<K, V, N>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = EnumMapVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}
