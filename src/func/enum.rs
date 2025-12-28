/// Represents the type of a function in the context of getter and setter methods.
///
/// # Variants
/// - `Get`: Represents a getter function.
/// - `GetMut`: Represents a mutable getter function.
/// - `Set`: Represents a setter function.
/// - `Debug`: Represents a debug function.
/// - `Unknown`: Represents an unknown or unspecified function type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FuncType {
    /// Represents a getter function.
    Get,
    /// Represents a mutable getter function.
    GetMut,
    /// Represents a setter function.
    Set,
    /// Represents a debug function.
    Debug,
    /// Represents an unknown or unspecified function type.
    Unknown,
}
