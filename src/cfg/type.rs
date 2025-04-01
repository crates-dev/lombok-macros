use crate::{func::r#type::FuncType, visibility::r#type::Visibility};

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
    pub(crate) func_type: FuncType,
    pub(crate) skip: bool,
    pub(crate) visibility: Visibility,
}
