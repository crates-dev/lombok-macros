use crate::*;

/// Provides default configuration values for the Config struct.
impl Default for Config {
    /// Returns a default `Config` instance with unknown function type, skip set to false, public visibility, no trait type, and default return type.
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
            trait_type: TraitType::None,
            return_type: ReturnType::Default,
        }
    }
}
