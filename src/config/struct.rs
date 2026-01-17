use crate::{func::*, visibility::*};

/// Represents the configuration for function types, visibility, trait parameters, return type, and skipping behavior.
///
/// This struct holds the configuration for function types (`FuncType`),
/// whether to skip processing (`skip`), the visibility of the function (`Visibility`),
/// the trait type for parameter conversion (`TraitType`), and the return type behavior (`ReturnType`).
///
/// # Fields
/// - `func_type`: A `FuncType` that specifies the function type.
/// - `skip`: A boolean flag indicating whether the function should be skipped during processing.
/// - `visibility`: A `Visibility` that defines the visibility of the function.
/// - `trait_type`: A `TraitType` that specifies the trait for parameter conversion.
/// - `return_type`: A `ReturnType` that specifies the return type behavior for getters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    /// A `FuncType` that specifies the function type.
    pub(crate) func_type: FuncType,
    /// A boolean flag indicating whether the function should be skipped during processing.
    pub(crate) skip: bool,
    /// A `Visibility` that defines the visibility of the function.
    pub(crate) visibility: Visibility,
    /// A `TraitType` that specifies the trait for parameter conversion.
    pub(crate) trait_type: TraitType,
    /// A `ReturnType` that specifies the return type behavior for getters.
    pub(crate) return_type: ReturnType,
}
