use crate::Enumerated;
use std::{
    array,
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// Creates an EnumMap with user-provided values.
/// ```
/// use enum_collections::{em, Enumerated, EnumMap};
/// #[derive(Enumerated)]
/// enum Letter {
///    A,
///    B,
/// }
///
/// let enum_map = em!(Letter, i32,  A => 42, B => 24);
/// assert_eq!(42, enum_map[Letter::A]);
/// assert_eq!(24, enum_map[Letter::B]);
/// ```
///
#[macro_export]
macro_rules! em {

    ($ktp:ty, $vtp:ty, $($x:ident=>$y:expr),* ) => {
        enum_collections::EnumMap::<$ktp, $vtp, {<$ktp>::SIZE}>::new_inspect(|letter| {
            match letter {
                $(<$ktp>::$x => $y,)*
            }
        })
    };

}

/// Initializes an EnumMap with default values for all variants not explicitly specified.
/// ```
/// use enum_collections::{em_default, Enumerated, EnumMap};
/// #[derive(Enumerated)]
/// enum Letter {
///    A,
///    B,
/// }
///
/// // One non-default value
/// let enum_map = em_default!(Letter, i32, A => 42);
/// assert_eq!(42, enum_map[Letter::A]);
/// assert_eq!(i32::default(), enum_map[Letter::B]);
///
/// // All default
///
/// let enum_map = em_default!(Letter, i32,);
/// assert_eq!(i32::default(), enum_map[Letter::A]);
/// assert_eq!(i32::default(), enum_map[Letter::B]);
/// ```
///
#[macro_export]
macro_rules! em_default {
    ($ktp:ty, $vtp:ty, $($x:ident=>$y:expr),* ) => {
        EnumMap::<$ktp, $vtp, {<$ktp>::SIZE}>::new_inspect(|letter| {
            match letter {
                $(<$ktp>::$x => $y,)*
                _ => Default::default(),
            }
        })
    };
}

/// Initializes an EnumMap with `Option::None` for all variants not explicitly specified.
///
/// ```
/// use enum_collections::{em_option, Enumerated, EnumMap};
/// #[derive(Enumerated)]
/// enum Letter {
///   A,
///   B,
/// }
///
/// let enum_map = em_option!(Letter, i32, A => 42);
/// assert_eq!(Some(42), enum_map[Letter::A]);
/// assert_eq!(None, enum_map[Letter::B]);
/// ```
#[macro_export]
macro_rules! em_option {
    ($ktp:ty, $vtp:ty, $($x:ident=>$y:expr),* ) => {
        EnumMap::<$ktp, Option<$vtp>, {<$ktp>::SIZE}>::new_inspect(|letter| {
            match letter {
                $(<$ktp>::$x => Some($y),)*
                _ => None,
            }
        })
    };
}

#[cfg(test)]
mod macro_test {
    use crate::{EnumMap, Enumerated};

    #[derive(Enumerated)]
    enum Letter {
        A,
        B,
    }

    #[test]
    fn test_macro() {
        let enum_map = em_default!(Letter, i32,  A=>42);
        assert_eq!(42, enum_map[Letter::A]);
        assert_eq!(i32::default(), enum_map[Letter::B]);
    }
}

/// A map of enum variants to values. EnumMap is a fixed-size map, where each variant of the enum
/// is mapped to a value. EnumMap is a a zero-cost abstraction over an array, where the index of the array
/// corresponds to the position of the variant in the enum.
///
/// Because it is a thin wrapper of an array, it is stack-allocated by default. Simply [std::boxed::Box]ing it
/// will move it to the heap, at the caller's discretion.
///
/// - Indexed by enum variants.
/// - IndexMut by enum variants.
/// - Debug if the enum is Debug.
/// - PartialEq if the value is PartialEq. Same for Eq.
///
/// Debug and Eq are optional features. They are enabled by default.
///
/// # Examples
///
/// ```
/// use enum_collections::{EnumMap, Enumerated, em_default, em};
///
/// #[derive(Enumerated)]
/// pub enum Letter {
///    A,
///    B,
/// }
///
///
/// // Indexing and mutation
/// let mut enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
/// assert_eq!(0, enum_map[Letter::A]);
/// enum_map[Letter::A] = 42;
/// assert_eq!(42, enum_map[Letter::A]);
///
/// // Construction using macros
/// // (Key type, Value type, Key=>Value pairs)
/// let enum_map = em!(Letter, i32,  A=>42, B=>24); // All values set explicitly
/// assert_eq!(42, enum_map[Letter::A]);
/// assert_eq!(24, enum_map[Letter::B]);
///
/// // (Key type, Value type, optional Key=>Value pairs)
/// let enum_map = em_default!(Letter, i32, A => 42); // Default used for missing values
/// assert_eq!(42, enum_map[Letter::A]);
/// assert_eq!(i32::default(), enum_map[Letter::B]);
///
/// let enum_map = em_default!(Letter, i32,); // All default
/// assert_eq!(i32::default(), enum_map[Letter::A]);
/// assert_eq!(i32::default(), enum_map[Letter::B]);
///
///
/// // Constructor with default values
/// let enum_map_default = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
/// assert_eq!(0, enum_map_default[Letter::A]);
/// assert_eq!(0, enum_map_default[Letter::B]);
///
/// // Convenience constructor for optional values
/// let mut enum_map_option = EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option();
/// assert_eq!(None, enum_map_option[Letter::A]);
/// assert_eq!(None, enum_map_option[Letter::B]);
/// enum_map_option[Letter::A] = Some(42);
/// assert_eq!(Some(42), enum_map_option[Letter::A]);
///
/// // Constructor with custom initialization
/// #[derive(PartialEq, Eq, Debug)]
/// struct Custom;
/// let enum_map = EnumMap::<Letter, Custom, { Letter::SIZE }>::new(|| Custom);
/// assert_eq!(Custom, enum_map[Letter::A]);
/// assert_eq!(Custom, enum_map[Letter::B]);
///
/// // Custom initialization function with enum variant (key) inspection
/// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_inspect(|letter| {
///     match letter {
///        Letter::A => 42,
///        Letter::B => 24,
///     }
/// });
/// assert_eq!(42, enum_map[Letter::A]);
/// assert_eq!(24, enum_map[Letter::B]);
///
/// // Debug
/// #[derive(Enumerated, Debug)]
/// pub enum LetterDebugDerived {
///    A,
///    B,
/// }
/// let enum_map_debug =
///     EnumMap::<LetterDebugDerived, i32, { LetterDebugDerived::SIZE }>::new(|| 42);
/// assert_eq!("{A: 42, B: 42}", format!("{:?}", enum_map_debug));
///
/// ```
pub struct EnumMap<K: Enumerated, V, const N: usize> {
    data: [V; N],
    _key: PhantomData<K>,
}

impl<K: Enumerated, V: Default, const N: usize> EnumMap<K, V, N> {
    /// Creates a new EnumMap with type's default values for each variant.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
    /// assert_eq!(0, enum_map[Letter::A]);
    /// assert_eq!(0, enum_map[Letter::B]);
    /// ```
    pub fn new_default() -> Self {
        Self {
            data: array::from_fn(|_| V::default()),
            _key: PhantomData,
        }
    }
    /// Sets all values to `V::default()`.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///   A,
    ///     B,
    /// }
    /// let mut enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
    /// enum_map.iter().for_each(|value| assert_eq!(42, *value));
    /// enum_map.clear_set_default();
    /// enum_map.iter().for_each(|value| assert_eq!(0, *value));
    /// ```
    ///
    pub fn clear_set_default(&mut self) {
        for idx in 0..self.data.len() {
            self.data[idx] = V::default();
        }
    }
}

impl<K: Enumerated, V, const N: usize> EnumMap<K, Option<V>, N> {
    /// Creates a new EnumMap with `Option::None` set for each variant.
    /// Convenience constructor over `EnumMap::new` for optional values.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option();
    /// assert_eq!(None, enum_map[Letter::A]);
    /// assert_eq!(None, enum_map[Letter::B]);
    /// ```
    pub fn new_option() -> Self {
        Self {
            data: array::from_fn(|_| None),
            _key: PhantomData,
        }
    }

    /// Clears the EnumMap and sets all values to `None`.
    ///
    /// This function iterates over each variant of the EnumMap and sets its value to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let mut enum_map = EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option();
    /// enum_map[Letter::A] = Some(10);
    /// enum_map[Letter::B] = Some(20);
    ///
    /// enum_map.clear_set_none();
    /// enum_map.iter().for_each(|value| assert_eq!(None, *value));
    ///
    /// ```
    pub fn clear_set_none(&mut self) {
        for idx in 0..self.data.len() {
            self.data[idx] = None;
        }
    }
}

impl<K: Enumerated, V, const N: usize> EnumMap<K, V, N> {
    /// Creates a new EnumMap where value of each variant is produced by the provided function
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
    /// assert_eq!(42, enum_map[Letter::A]);
    /// assert_eq!(42, enum_map[Letter::B]);
    ///
    /// ```
    pub fn new(default: fn() -> V) -> Self {
        Self {
            data: array::from_fn(|_| default()),
            _key: PhantomData,
        }
    }

    /// Iterates over each variant of the EnumMap and sets its value to the value provided by the `val_provider` function.
    ///
    /// Unlike [Self::new], this function does not allocate a new EnumMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///   A,
    ///   B,
    /// }
    ///
    /// let mut enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
    /// enum_map.set_all(|| 24);
    /// enum_map.iter().for_each(|value| assert_eq!(24, *value));
    /// ```
    pub fn set_all(&mut self, val_provider: fn() -> V) {
        for idx in 0..self.data.len() {
            self.data[idx] = val_provider();
        }
    }

    /// Iterates over the EnumMap's key-value pairs.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated, Debug)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
    /// for (_letter, value) in enum_map.iter_kv() {
    ///    assert_eq!(42, *value);
    /// }
    ///
    /// ```
    #[cfg(feature = "variants")]
    pub fn iter_kv(&self) -> std::iter::Zip<std::slice::Iter<'_, K>, std::slice::Iter<'_, V>> {
        K::VARIANTS.iter().zip(self.data.iter())
    }

    /// Iterates over the EnumMap's values.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated, Debug)]
    /// pub enum Letter {
    ///    A,
    ///    B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
    /// for value in enum_map.iter() {
    ///    assert_eq!(42, *value);
    /// }
    ///
    /// ```
    #[cfg(feature = "variants")]
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.data.iter()
    }

    /// Creates a new EnumMap where value of each variant is produced by the provided function.
    /// The function receives the enum variant being initialized for inspection.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///   A,
    ///  B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_inspect(|letter| {
    ///    match letter {
    ///       Letter::A => 42,
    ///      Letter::B => 24,
    ///    }
    /// });
    /// assert_eq!(42, enum_map[Letter::A]);
    /// assert_eq!(24, enum_map[Letter::B]);
    ///
    /// ```
    #[cfg(feature = "variants")]
    pub fn new_inspect(default: fn(&K) -> V) -> Self {
        let init_fn = |index| {
            // Finds the enum variant by its index, as the array is sorted by discriminants in ascending order.
            default(&K::VARIANTS[index])
        };
        Self {
            data: array::from_fn(init_fn),
            _key: PhantomData,
        }
    }
}

impl<K: Enumerated, V: Copy, const N: usize> EnumMap<K, V, N> {
    /// Creates a new EnumMap where value of each variant is the provided value.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    /// #[derive(Enumerated)]
    /// pub enum Letter {
    ///   A,
    ///   B,
    /// }
    ///
    /// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_with_all(42);
    /// assert_eq!(42, enum_map[Letter::A]);
    /// assert_eq!(42, enum_map[Letter::B]);
    ///
    /// ```
    pub const fn new_with_all(value: V) -> Self {
        Self {
            data: [value; N],
            _key: PhantomData,
        }
    }
}

/// Allows indexing of EnumMap.
///
/// ```
///  use enum_collections::{EnumMap, Enumerated};
///
/// #[derive(Enumerated)]
/// pub enum LetterEqDerived {
///     A,
///     B,
/// }
///
/// let enum_map = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
/// assert_eq!(42, enum_map[LetterEqDerived::A]);
/// assert_eq!(42, enum_map[LetterEqDerived::B]);
/// ```
impl<K: Enumerated, V, const N: usize> Index<K> for EnumMap<K, V, N> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        &self.data[key.position()]
    }
}

/// Allows mutable indexing of EnumMap.
///
///
/// ```
///  use enum_collections::{EnumMap, Enumerated};
///
/// #[derive(Enumerated)]
/// pub enum LetterEqDerived {
///     A,
///     B,
/// }
///
/// let mut enum_map = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new_default();
/// assert_eq!(0, enum_map[LetterEqDerived::A]);
/// enum_map[LetterEqDerived::A] = 42;
/// assert_eq!(42, enum_map[LetterEqDerived::A]);
///
/// ```
impl<K: Enumerated, V, const N: usize> IndexMut<K> for EnumMap<K, V, N> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.data[key.position()]
    }
}

/// Implements Debug for EnumMap. EnumMap is printed as a map of enum variants to their values.
///
/// ```
///
/// use enum_collections::{EnumMap, Enumerated};
/// #[derive(Enumerated, Debug)]
/// enum Letter {
///    A,
///    B,
/// }
///
/// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
/// assert_eq!("{A: 42, B: 42}", format!("{:?}", enum_map));
/// ```
///
#[cfg(feature = "debug")]
impl<K: Enumerated + Debug, V: Debug, const N: usize> std::fmt::Debug for EnumMap<K, V, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(
                K::VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| (variant, &self.data[index])),
            )
            .finish()
    }
}

#[cfg(feature = "eq")]
mod eq {
    use super::{EnumMap, Enumerated};

    /// Implements PartialEq for EnumMap. Two enum maps are PartialEq if for each enum variant, the value is the same.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    ///
    /// #[derive(Enumerated, Eq, PartialEq)]
    /// pub enum LetterEqDerived {
    ///     A,
    ///     B,
    /// }
    /// let enum_map = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
    /// let same_map = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
    /// assert!(enum_map == same_map);
    ///
    /// let different_map = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new_default();
    /// assert!(enum_map != different_map);
    /// ```
    impl<K: Enumerated, V: PartialEq, const N: usize> PartialEq for EnumMap<K, V, N> {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }

    /// Marks EnumMap as Eq. Two enum maps are Eq if for each enum variant, the value is the same.
    ///
    /// ```
    /// use enum_collections::{EnumMap, Enumerated};
    ///
    /// #[derive(Enumerated, Eq, PartialEq)]
    /// pub enum LetterEqDerived {
    ///     A,
    ///     B,
    /// }
    /// let first = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
    /// let second = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
    /// let third = EnumMap::<LetterEqDerived, i32, { LetterEqDerived::SIZE }>::new(|| 42);
    /// // Reflexive
    /// assert!(first == first);
    /// // Symmetric
    /// assert!(first == second);
    /// assert!(second == first);
    /// // Transitive
    /// assert!(second == third);
    /// assert!(first == third);
    /// ```
    impl<K: Enumerated, V: Eq, const N: usize> Eq for EnumMap<K, V, N> {}
}

/// Implements Clone for EnumMap. Clones the EnumMap by cloning each value. Requires the value to be `Clone`.
///
/// ```
/// use enum_collections::{EnumMap, Enumerated};
///
/// #[derive(Enumerated, Debug)]
/// pub enum Letter {
///     A,
///     B,
/// }
/// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
/// let cloned = enum_map.clone();
/// assert_eq!(enum_map, cloned);
/// ```
impl<K: Enumerated, V: Clone, const N: usize> Clone for EnumMap<K, V, N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _key: PhantomData,
        }
    }
}

/// Implements Copy for EnumMap, provided the value type `V` also implements `Copy`.
///
/// ```
///
/// use enum_collections::{EnumMap, Enumerated};
/// #[derive(Enumerated, Debug)]
/// pub enum Letter {
///   A,
///   B,
/// }
/// let enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new(|| 42);
/// let copied = enum_map;
/// assert_eq!(enum_map, copied);
/// ```
impl<K: Enumerated, V: Copy, const N: usize> Copy for EnumMap<K, V, N> {}

#[cfg(test)]
mod tests {
    use crate::enummap::EnumMap;
    use crate::Enumerated;
    /// No Debug derived on purpose, the crate must be usable without [std::fmt::Debug] derived
    /// for the enum.
    #[derive(Enumerated, Debug)]
    pub(super) enum Letter {
        A,
        B,
    }

    #[test]
    fn index() {
        let mut enum_map = EnumMap::<Letter, i32, { Letter::SIZE }>::new_default();
        assert_eq!(0, enum_map[Letter::A]);
        enum_map[Letter::A] = 42;
        assert_eq!(42, enum_map[Letter::A]);
        assert_eq!(i32::default(), enum_map[Letter::B]);
    }

    #[test]
    fn constructor_option() {
        let mut enum_map = EnumMap::<Letter, Option<i32>, { Letter::SIZE }>::new_option();
        assert_eq!(None, enum_map[Letter::A]);
        assert_eq!(None, enum_map[Letter::B]);

        enum_map[Letter::A] = Some(42);
        assert_eq!(Some(42), enum_map[Letter::A]);
    }

    #[test]
    fn non_default_type() {
        #[derive(PartialEq, Eq, Debug)]
        struct NonDefault;
        let enum_map = EnumMap::<Letter, NonDefault, { Letter::SIZE }>::new(|| NonDefault);
        assert_eq!(NonDefault, enum_map[Letter::A]);
        assert_eq!(NonDefault, enum_map[Letter::B]);
    }

    /// Safeguard test. Nothing inside the EnumMap should prevent from moving it to the heap.
    #[test]
    fn heap_allocation() {
        let boxed_map = Box::new(EnumMap::<Letter, i32, { Letter::SIZE }>::new_default());
        assert!(EnumMap::<Letter, i32, { Letter::SIZE }>::new_default() == *boxed_map);
    }

    #[cfg(feature = "variants")]
    mod variants {
        use super::*;

        #[test]
        fn variants() {
            assert_eq!(2, Letter::VARIANTS.len());
            Letter::VARIANTS
                .iter()
                .for_each(|letter| println!("{:?}", letter));
        }
    }

    #[cfg(feature = "debug")]
    mod debug {
        use crate::{EnumMap, Enumerated};

        /// A dedicated enum with [std::fmt::Debug] derived, to test compilation and usability both
        /// with and without `Debug` implemented.
        #[derive(Enumerated, Debug)]
        pub(super) enum LetterDebugDerived {
            A,
            B,
        }

        #[test]
        fn debug() {
            let enum_map =
                EnumMap::<LetterDebugDerived, i32, { LetterDebugDerived::SIZE }>::new(|| 42);
            assert_eq!("{A: 42, B: 42}", format!("{:?}", enum_map));
        }
    }
}
