use crate::*;

/// Represents the configuration for function types, visibility, return type, and skipping behavior.
///
/// This struct holds the configuration for function types (`FuncType`),
/// whether to skip processing (`skip`), the visibility of the function (`Visibility`),
/// the return type behavior (`ReturnType`), and optional custom parameter type specification.
///
/// # Fields
/// - `func_type` - A `FuncType` that specifies the function type.
/// - `skip` - A boolean flag indicating whether the function should be skipped during processing.
/// - `visibility` - A `Visibility` that defines the visibility of the function.
/// - `return_type` - A `ReturnType` that specifies the return type behavior for getters.
/// - `param_type_override` - Optional custom parameter type to use instead of deriving from field type.
#[derive(Debug, Clone)]
pub(crate) struct Config {
    /// A `FuncType` that specifies the function type.
    pub(crate) func_type: FuncType,
    /// A boolean flag indicating whether the function should be skipped during processing.
    pub(crate) skip: bool,
    /// A `Visibility` that defines the visibility of the function.
    pub(crate) visibility: Visibility,
    /// A `ReturnType` that specifies the return type behavior for getters.
    pub(crate) return_type: ReturnType,
    /// Optional custom parameter type to use instead of deriving from field type.
    pub(crate) param_type_override: Option<TokenStream2>,
}
