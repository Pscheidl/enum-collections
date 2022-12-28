mod enumerated;
mod enummap;
mod enumtable;

pub use crate::enumerated::Enumerated;
pub use crate::enummap::EnumMap;
pub use crate::enumtable::EnumTable;
pub use enum_collections_macros::enum_collections;

#[cfg(test)]
mod tests {
    use crate::Enumerated;
    use enum_collections_macros::enum_collections;

    #[test]
    fn test() {
        #[enum_collections]
        enum Letter {
            A,
            B,
        }

        assert_eq!(0, Letter::A.position());
        assert_eq!(1, Letter::B.position());
        assert_eq!(2, Letter::len());
    }
}
