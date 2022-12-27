mod enummap;

pub use crate::enummap::EnumMap;
pub use crate::enummap::Enumerated;
pub use enummap_macros::enummap;

#[cfg(test)]
mod tests {
    use crate::enummap;

    #[test]
    fn test() {
        use crate::Enumerated;

        #[enummap]
        enum Letter {
            A,
            B,
        }

        assert_eq!(0, Letter::A.position());
        assert_eq!(1, Letter::B.position());
        assert_eq!(2, Letter::len());
    }
}
