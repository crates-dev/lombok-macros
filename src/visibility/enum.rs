/// Represents the visibility of an item in a Rust module.
///
/// # Variants
/// - `Public` - The item is visible to all modules.
/// - `PublicCrate` - The item is visible only within the current crate.
/// - `PublicSuper` - The item is visible to the parent module and its descendants.
/// - `Private` - The item is visible only within the current module.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Visibility {
    #[default]
    Public,
    PublicCrate,
    PublicSuper,
    Private,
}
