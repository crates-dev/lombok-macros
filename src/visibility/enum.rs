/// Represents the visibility of an item in a Rust module.
///
/// # Variants
/// - `Public`: The item is visible to all modules.
/// - `PublicCrate`: The item is visible only within the current crate.
/// - `PublicSuper`: The item is visible to the parent module and its descendants.
/// - `Private`: The item is visible only within the current module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Visibility {
    Public,
    PublicCrate,
    PublicSuper,
    Private,
}
