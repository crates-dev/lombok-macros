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

/// Represents the return type behavior for getter methods.
///
/// This enum defines how getter methods should return values,
/// controlling whether they return references, cloned values, or use default behavior.
///
/// # Variants
/// - `Reference`: Returns a reference to the field value (`&T`).
/// - `Clone`: Returns a cloned copy of the field value (`T`).
/// - `Copy`: Returns a copy of the field value for types implementing Copy trait (`self.field`).
/// - `Deref`: Returns a dereferenced value of the field (`*field`), with match control for Option/Result.
/// - `Default`: Uses default behavior (reference for non-Option/Result, cloned for Option/Result).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReturnType {
    /// Returns a reference to the field value (`&T`).
    Reference,
    /// Returns a cloned copy of the field value (`T`).
    Clone,
    /// Returns a copy of the field value for types implementing Copy trait (`self.field`).
    Copy,
    /// Returns a dereferenced value of the field (`*field`), with match control for Option/Result.
    Deref,
    /// Uses default behavior (reference for non-Option/Result, cloned for Option/Result).
    Default,
}
