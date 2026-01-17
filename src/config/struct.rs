use crate::{func::*, visibility::*};

/// Represents the configuration for function types, visibility, trait parameters, and skipping behavior.
///
/// This struct holds the configuration for function types (`FuncType`),
/// whether to skip processing (`skip`), the visibility of the function (`Visibility`),
/// and the trait type for parameter conversion (`TraitType`).
///
/// # Fields
/// - `func_type`: A `FuncType` that specifies the function type.
/// - `skip`: A boolean flag indicating whether the function should be skipped during processing.
/// - `visibility`: A `Visibility` that defines the visibility of the function.
/// - `trait_type`: A `TraitType` that specifies the trait for parameter conversion.
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
}
