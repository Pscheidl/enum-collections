mod enummap;

pub use crate::enummap::Enumerated;
pub use enummap_macros::enummap;

#[cfg(test)]
mod tests {
    use crate::enummap;

    #[test]
    fn test() {
        use crate::Enumerated;

        #[enummap]
        enum Letters {
            A,
            B,
        }

        assert_eq!(0, Letters::A.position());
        assert_eq!(1, Letters::B.position());
        assert_eq!(2, Letters::len());
    }
}
