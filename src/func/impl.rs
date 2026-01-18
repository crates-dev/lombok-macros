use crate::*;

/// Implements the `FromStr` trait for `FuncType` to parse string representations into `FuncType` variants.
impl FromStr for FuncType {
    type Err = String;

    /// Parses a string slice into a `FuncType`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to parse.
    ///
    /// # Returns
    ///
    /// - `Result<FuncType, String>` - Ok containing the `FuncType` if parsing is successful,
    ///   or an Err containing a String error message if parsing fails.
    #[inline(always)]
    fn from_str(s: &str) -> Result<FuncType, String> {
        match s {
            GET => Ok(FuncType::Get),
            GET_MUT => Ok(FuncType::GetMut),
            SET => Ok(FuncType::Set),
            DEBUG => Ok(FuncType::Debug),
            NEW => Ok(FuncType::New),
            _ => Ok(FuncType::Unknown),
        }
    }
}

impl FuncType {
    /// Checks if the `FuncType` is `Get`.
    ///
    /// # Arguments
    ///
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    ///
    /// - `bool` - if the `FuncType` is `Get`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_get(&self) -> bool {
        *self == FuncType::Get
    }

    /// Checks if the `FuncType` is `GetMut`.
    ///
    /// # Arguments
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` is `GetMut`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_get_mut(&self) -> bool {
        *self == FuncType::GetMut
    }

    /// Checks if the `FuncType` is `Set`.
    ///
    /// # Arguments
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` is `Set`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_set(&self) -> bool {
        *self == FuncType::Set
    }

    /// Checks if the `FuncType` is `Debug`.
    ///
    /// # Arguments
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` is `Debug`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_debug(&self) -> bool {
        *self == FuncType::Debug
    }

    /// Checks if the `FuncType` is `New`.
    ///
    /// # Arguments
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` is `New`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_new(&self) -> bool {
        *self == FuncType::New
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Arguments
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` is `Unknown`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Arguments
    /// - `func_type_str` - The string slice representing the function type to check.
    ///
    /// # Returns
    /// - `bool` - if the `FuncType` parsed from the string is not `Unknown`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_known(func_type_str: &str) -> bool {
        let func_type: FuncType = func_type_str.parse::<FuncType>().unwrap_or_default();
        func_type != Self::Unknown
    }
}

impl ReturnType {
    /// Checks if the return type is `Default`.
    ///
    /// # Returns
    /// - `bool` - if the return type is `Default`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_default(&self) -> bool {
        *self == ReturnType::Default
    }

    /// Checks if the given string is a known return type.
    ///
    /// # Arguments
    /// - `&str` - The string to check.
    ///
    /// # Returns
    /// - `bool` - if the string is a known return type; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_known(s: &str) -> bool {
        matches!(s, REFERENCE | CLONE | COPY | DEREF)
    }
}

impl Default for ReturnType {
    /// Returns the default return type (`Default`).
    ///
    /// # Returns
    ///
    /// - `ReturnType` - The default return type.
    #[inline(always)]
    fn default() -> Self {
        ReturnType::Default
    }
}

impl FromStr for ReturnType {
    type Err = String;

    /// Parses a string slice into a `ReturnType`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to parse.
    ///
    /// # Returns
    ///
    /// - `Result<ReturnType, String>` - Ok containing the `ReturnType` if parsing is successful,
    ///   or an Err containing a String error message if parsing fails.
    #[inline(always)]
    fn from_str(s: &str) -> Result<ReturnType, String> {
        match s {
            REFERENCE => Ok(ReturnType::Reference),
            CLONE => Ok(ReturnType::Clone),
            COPY => Ok(ReturnType::Copy),
            DEREF => Ok(ReturnType::Deref),
            _ => Ok(ReturnType::Default),
        }
    }
}

/// Implements the `From<&str>` trait for `ParameterType` to parse string representations into `ParameterType` variants.
impl From<&str> for ParameterType {
    /// Parses a string slice into a `ParameterType`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to parse.
    ///
    /// # Returns
    ///
    /// - `ParameterType` - The corresponding `ParameterType` variant.
    #[inline(always)]
    fn from(type_str: &str) -> Self {
        let trimmed: &str = type_str.trim();
        if trimmed.starts_with(AS_REF_PREFIX) && trimmed.ends_with(CLOSE_BRACKET) {
            ParameterType::AsRef
        } else if trimmed.starts_with(INTO_PREFIX) && trimmed.ends_with(CLOSE_BRACKET) {
            ParameterType::Into
        } else if trimmed.starts_with(AS_MUT_PREFIX) && trimmed.ends_with(CLOSE_BRACKET) {
            ParameterType::AsMut
        } else if trimmed.starts_with(DEREF_PREFIX) && trimmed.ends_with(CLOSE_BRACKET) {
            ParameterType::Deref
        } else if trimmed.contains(OPEN_BRACKET)
            && trimmed.contains(CLOSE_BRACKET)
            && !trimmed.starts_with(IMPL_PREFIX)
        {
            ParameterType::Custom(trimmed.to_string())
        } else {
            ParameterType::Direct
        }
    }
}
