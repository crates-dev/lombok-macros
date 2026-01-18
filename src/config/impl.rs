use crate::*;

/// Provides default configuration values for the Config struct.
impl Default for Config {
    /// Returns a default `Config` instance with unknown function type, empty skip and added flag sets, public visibility, and default return type.
    ///
    /// # Returns
    ///
    /// - `Config` - A new `Config` instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknown,
            visibility: Visibility::Public,
            return_type: ReturnType::Default,
            param_type_override: None,
            skip_flags: HashSet::new(),
            added_flags: HashSet::new(),
        }
    }
}
