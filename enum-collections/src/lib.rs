//!# Enum collections
//!
//! See [EnumMap] for usage details.
//!
//! A map of enum variants to values. EnumMap is a fixed-size map, where each variant of the enum is mapped to a value.
//! This implementation of EnumMap uses **safe Rust** only and is a a zero-cost abstraction over an array (**const-sized**),
//! where the index of the array corresponds to the position of the variant in the enum.
//!
//! Because it is a thin wrapper over an array, it is stack-allocated by default. Simply `std::boxed::Box`ing it will move it to the heap, at the caller's discretion.
//!- Indexed by enum variants.
//!- IndexMut by enum variants.
//!- Debug if the enum is Debug.
//!- PartialEq if the value is PartialEq. Same for Eq.
//!
//!Debug and Eq are optional features. They are enabled by default.
//!
//!
mod enumerated;
mod enummap;

pub use crate::enumerated::Enumerated;
pub use crate::enummap::EnumMap;
pub use enum_collections_macros::Enumerated;

#[cfg(test)]
mod tests {
    use crate::Enumerated;

    #[test]
    fn test_derive_macro() {
        #[derive(Enumerated)]
        enum Letter {
            A,
            B,
        }

        assert_eq!(0, Letter::A.position());
        assert_eq!(1, Letter::B.position());
        assert_eq!(2, Letter::SIZE);
    }
}
