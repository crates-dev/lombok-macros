use crate::*;
use std::collections::HashSet;

/// Represents the configuration for function types, visibility, return type, and skipping behavior.
///
/// This struct holds the configuration for function types (`FuncType`),
/// sets of flags for functions to skip or that have been added, the visibility of the function (`Visibility`),
/// the return type behavior (`ReturnType`), and optional custom parameter type specification.
///
/// # Fields
/// - `func_type` - A `FuncType` that specifies the function type.
/// - `visibility` - A `Visibility` that defines the visibility of the function.
/// - `return_type` - A `ReturnType` that specifies the return type behavior for getters.
/// - `param_type_override` - Optional custom parameter type to use instead of deriving from field type.
/// - `skip_flags` - A set of `SkipFlag` indicating which function types should be skipped.
/// - `added_flags` - A set of `AddedFlag` indicating which function types have been added.
#[derive(Clone, Debug, Default)]
pub(crate) struct Config {
    /// A `FuncType` that specifies the function type.
    pub(crate) func_type: FuncType,
    /// A `Visibility` that defines the visibility of the function.
    pub(crate) visibility: Visibility,
    /// A `ReturnType` that specifies the return type behavior for getters.
    pub(crate) return_type: ReturnType,
    /// Optional custom parameter type to use instead of deriving from field type.
    pub(crate) param_type_override: Option<TokenStream2>,
    /// A set of `FuncType` indicating which function types should be skipped.
    pub(crate) skip_flags: HashSet<FuncType>,
    /// A set of `FuncType` indicating which function types have been added.
    pub(crate) added_flags: HashSet<FuncType>,
}
