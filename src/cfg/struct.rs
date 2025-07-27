use crate::{func::*, visibility::*};

/// Represents the configuration for function types, visibility, and skipping behavior.
///
/// This struct holds the configuration for function types (`FuncType`),
/// whether to skip processing (`skip`), and the visibility of the function (`Visibility`).
///
/// # Fields
/// - `func_type`: A `FuncType` that specifies the function type (e.g., `Get`, `Set`, or `Unknown`).
/// - `skip`: A boolean flag indicating whether the function should be skipped during processing.
/// - `visibility`: A `Visibility` that defines the visibility of the function (e.g., `Public`, `Private`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cfg {
    /// A `FuncType` that specifies the function type (e.g., `Get`, `Set`, or `Unknown`).
    pub(crate) func_type: FuncType,
    /// A boolean flag indicating whether the function should be skipped during processing.
    pub(crate) skip: bool,
    /// A `Visibility` that defines the visibility of the function (e.g., `Public`, `Private`).
    pub(crate) visibility: Visibility,
}
