use crate::*;

/// Provides default implementation for FuncType.
impl Default for FuncType {
    /// Returns the default value for FuncType, which is Unknown.
    fn default() -> Self {
        Self::Unknown
    }
}

/// Implements the `FromStr` trait for `FuncType` to parse string representations into `FuncType` variants.
impl FromStr for FuncType {
    type Err = String;

    /// Parses a string slice into a `FuncType`.
    ///
    /// # Arguments
    ///
    /// - `s`: The string slice to parse.
    ///
    /// # Returns
    ///
    /// - `Result<FuncType, String>`: Ok containing the `FuncType` if parsing is successful,
    ///   or an Err containing a String error message if parsing fails.
    fn from_str(s: &str) -> Result<FuncType, std::string::String> {
        match s {
            GET => Ok(FuncType::Get),
            GET_MUT => Ok(FuncType::GetMut),
            SET => Ok(FuncType::Set),
            DEBUG => Ok(FuncType::Debug),
            _ => Ok(FuncType::Unknown),
        }
    }
}

impl FuncType {
    /// Checks if the `FuncType` is `Get`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Get`; otherwise, `false`.
    #[inline(always)]
    pub fn is_get(&self) -> bool {
        *self == FuncType::Get
    }

    /// Checks if the `FuncType` is `GetMut`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `GetMut`; otherwise, `false`.
    #[inline(always)]
    pub fn is_get_mut(&self) -> bool {
        *self == FuncType::GetMut
    }

    /// Checks if the `FuncType` is `Set`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Set`; otherwise, `false`.
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == FuncType::Set
    }

    /// Checks if the `FuncType` is `Debug`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Debug`; otherwise, `false`.
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == FuncType::Debug
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Unknown`; otherwise, `false`.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Parameters
    /// - `func_type_str`: The string slice representing the function type to check.
    ///
    /// # Returns
    /// - `true` if the `FuncType` parsed from the string is not `Unknown`; otherwise, `false`.
    #[inline(always)]
    pub fn is_known(func_type_str: &str) -> bool {
        let func_type: FuncType = func_type_str.parse::<FuncType>().unwrap_or_default();
        func_type != Self::Unknown
    }
}
