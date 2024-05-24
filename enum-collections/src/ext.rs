//! Implementations of the [Enumerated] trait for common data types.

use crate::Enumerated;

impl Enumerated for bool {
    const SIZE: usize = 2;
    const VARIANTS: &'static [Self] = &[false, true];

    fn position(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::{em_default, EnumMap, Enumerated};

    #[test]
    fn test_bool() {
        assert_eq!(0, false.position());
        assert_eq!(1, true.position());
        assert_eq!(2, bool::SIZE);

        let map = EnumMap::<bool, i32, { bool::SIZE }>::new_inspect(|variant| match variant {
            false => 42,
            true => 24,
        });

        assert_eq!(42, map[false]);
        assert_eq!(24, map[true]);

        let map = em_default!(bool, i32,);
        for variant in bool::VARIANTS {
            assert_eq!(0, map[*variant]);
        }
    }
}
