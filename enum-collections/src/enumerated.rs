/// Provides means to map enum  values to positions in arrays backing an EnumMap/EnumTable.
/// Not intended to be implemented by hand. Annotating an enum with the `#[enum_collections]`
/// attribute macro is preferred.
///
/// ```
///  use enum_collections::{enum_collections, Enumerated};
/// #[enum_collections]
/// enum Letter {
///     A,
///     B,
/// }
/// ```
pub trait Enumerated {
    /// Maps an enum to a unique position in an array.
    fn position(&self) -> usize;
    /// Total number of values in an Enum.
    fn len() -> usize;
}
