use crate::*;

/// Provides default configuration values for the Cfg struct.
impl Default for Cfg {
    /// Returns a default `Cfg` instance with unknown function type, skip set to false, and public visibility.
    ///
    /// # Returns
    ///
    /// - `Cfg` - A new `Cfg` instance with default values.
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknown,
            skip: false,
            visibility: Visibility::Public,
        }
    }
}
