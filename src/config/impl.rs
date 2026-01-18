use crate::*;

/// Provides default configuration values for the Config struct.
impl Default for Config {
    /// Returns a default `Config` instance with unknown function type, skip set to false, public visibility, and default return type.
    ///
    /// # Returns
    ///
    /// - `Config` - A new `Config` instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknown,
            skip: false,
            visibility: Visibility::Public,
            return_type: ReturnType::Default,
            param_type_override: None,
        }
    }
}
