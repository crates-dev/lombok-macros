/// Constant for the "get" function type.
pub const GET: &str = "get";

/// Constant for the "get_mut" function type.
pub const GET_MUT: &str = "get_mut";

/// Constant for the "set" function type.
pub const SET: &str = "set";

/// Constant for the "debug" attribute.
pub const DEBUG: &str = "debug";

/// Constant for the "new" function type.
pub const NEW: &str = "new";

/// Constant for the "skip" attribute.
pub const SKIP: &str = "skip";

/// Constant for the "pub" visibility modifier.
pub const PUB: &str = "pub";

/// Constant for private visibility.
pub const PRIVATE: &str = "private";

/// Constant for the "crate" visibility modifier.
pub const CRATE: &str = "crate";

/// Constant for the "pub(crate)" visibility modifier.
pub const PUB_CRATE: &str = "pub(crate)";

/// Constant for the "super" visibility modifier.
pub const SUPER: &str = "super";

/// Constant for the "pub(super)" visibility modifier.
pub const PUB_SUPER: &str = "pub(super)";

/// Constant for return clone type.
pub const CLONE: &str = "clone";

/// Constant for return copy type.
pub const COPY: &str = "copy";

/// Constant for return deref type.
pub const DEREF: &str = "deref";

/// Constant for type specification.
pub const CUSTOM_TYPE: &str = "type";

/// Constant for AsRef trait bound prefix.
pub const AS_REF_PREFIX: &str = "AsRef<";

/// Constant for Into trait bound prefix.
pub const INTO_PREFIX: &str = "Into<";

/// Constant for AsMut trait bound prefix.
pub const AS_MUT_PREFIX: &str = "AsMut<";

/// Constant for Deref trait bound prefix.
pub const DEREF_PREFIX: &str = "Deref<";

/// Constant for impl keyword prefix.
pub const IMPL_PREFIX: &str = "impl ";

/// Constant for opening angle bracket character.
pub const OPEN_BRACKET: char = '<';

/// Constant for closing angle bracket character.
pub const CLOSE_BRACKET: char = '>';
