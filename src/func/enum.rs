/// Represents the type of a function in the context of getter and setter methods.
///
/// # Variants
/// - `Get`: Represents a getter function.
/// - `GetMut`: Represents a mutable getter function.
/// - `Set`: Represents a setter function.
/// - `Debug`: Represents a debug function.
/// - `New`: Represents a constructor function.
/// - `Unknown`: Represents an unknown or unspecified function type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FuncType {
    /// Represents a getter function.
    Get,
    /// Represents a mutable getter function.
    GetMut,
    /// Represents a setter function.
    Set,
    /// Represents a debug function.
    Debug,
    /// Represents a constructor function.
    New,
    /// Represents an unknown or unspecified function type.
    #[default]
    Unknown,
}

/// Represents the different trait types that can be used for setter parameters.
///
/// This enum defines the trait types that can be specified as parameters
/// for setter methods to enable more flexible type conversions.
///
/// # Variants
/// - `AsRef`: Uses `AsRef<T>` trait for parameter conversion.
/// - `Into`: Uses `Into<T>` trait for parameter conversion.
/// - `None`: No trait conversion (default behavior).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TraitType {
    /// Uses `AsRef<T>` trait for parameter conversion.
    AsRef,
    /// Uses `Into<T>` trait for parameter conversion.
    Into,
    /// No trait conversion (default behavior).
    None,
}
