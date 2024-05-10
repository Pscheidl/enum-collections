/// Provides means to map enum values to positions in arrays backing an EnumMap/EnumTable.
/// Not intended to be implemented by hand. Deriving it with `#[derive(Enumerated)]`
/// attribute macro is preferred.
///
/// ```
/// use enum_collections::Enumerated;
/// #[derive(Enumerated)]
/// enum Letter {
///     A,
///     B,
/// }
///
/// assert_eq!(Letter::SIZE, 2);
/// ```
pub trait Enumerated: Sized + 'static {
    /// Maps an enum to a unique position in an array.
    fn position(self) -> usize;
    const SIZE: usize = 0;
    /// All variants of the enum. Sorted by their discriminants ASC.
    #[cfg(feature = "variants")]
    const VARIANTS: &'static [Self];
}
