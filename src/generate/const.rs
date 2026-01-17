/// Error message indicating that a field must have a name.
pub(crate) const FIELD_SHOULD_HAVE_A_NAME: &str = "Field should have a name";

/// Prefix for getter methods.
pub(crate) const GET_METHOD_PREFIX: &str = "get_";

/// Prefix for mutable getter methods.
pub(crate) const GET_MUT_METHOD_PREFIX: &str = "get_mut_";

/// Prefix for setter methods.
pub(crate) const SET_METHOD_PREFIX: &str = "set_";

/// Error message indicating that #[derive(Data)] is only supported for structs.
pub(crate) const UNSUPPORTED_DATA_DERIVE: &str = "#[derive(Data)] is only supported for structs.";

/// Error message indicating that #[derive(New)] is only supported for structs.
pub(crate) const UNSUPPORTED_NEW_DERIVE: &str = "#[derive(New)] is only supported for structs.";

/// The string prefix used for raw identifiers.
pub(crate) const RAW_IDENT_PREFIX: &str = "r#";

/// The Option type identifier.
pub(crate) const OPTION_TYPE: &str = "Option";

/// The try_get method prefix.
pub(crate) const TRY_GET_METHOD_PREFIX: &str = "try_";
