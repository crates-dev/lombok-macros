/// Represents the type of a function in the context of getter and setter methods.
///
/// # Variants
/// - `Get`: Represents a getter function.
/// - `Set`: Represents a setter function.
/// - `Unknow`: Represents an unknown or unspecified function type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncType {
    Get,
    Set,
    Unknow,
}
