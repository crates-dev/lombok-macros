use crate::*;

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
    #[inline(always)]
    fn from_str(s: &str) -> Result<FuncType, std::string::String> {
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
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Get`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_get(&self) -> bool {
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
    pub(crate) fn is_get_mut(&self) -> bool {
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
    pub(crate) fn is_set(&self) -> bool {
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
    pub(crate) fn is_debug(&self) -> bool {
        *self == FuncType::Debug
    }

    /// Checks if the `FuncType` is `New`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `New`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_new(&self) -> bool {
        *self == FuncType::New
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Parameters
    /// - `self` - The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Unknown`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_unknown(&self) -> bool {
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
    pub(crate) fn is_known(func_type_str: &str) -> bool {
        let func_type: FuncType = func_type_str.parse::<FuncType>().unwrap_or_default();
        func_type != Self::Unknown
    }
}

impl Default for TraitType {
    /// Returns the default trait type (None).
    ///
    /// # Returns
    ///
    /// - `TraitType` - The default trait type (None).
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for TraitType {
    type Err = ();

    /// Parses a string into a TraitType.
    ///
    /// # Arguments
    ///
    /// - `s` - The string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<TraitType, ()>` - The parsed trait type or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AsRef" => Ok(Self::AsRef),
            "Into" => Ok(Self::Into),
            _ => Err(()),
        }
    }
}

impl TraitType {
    /// Checks if the given string is a known trait type.
    ///
    /// # Arguments
    ///
    /// - `s` - The string to check.
    ///
    /// # Returns
    ///
    /// - `bool` - true if the string is a known trait type, false otherwise.
    pub(crate) fn is_known(s: &str) -> bool {
        matches!(s, "AsRef" | "Into" | "as_ref" | "into")
    }
}

impl ReturnType {
    /// Checks if the return type is `Default`.
    ///
    /// # Returns
    /// - `true` if the return type is `Default`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_default(&self) -> bool {
        *self == ReturnType::Default
    }

    /// Checks if the given string is a known return type.
    ///
    /// # Parameters
    /// - `&str` - The string to check.
    ///
    /// # Returns
    /// - `true` if the string is a known return type; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_known(s: &str) -> bool {
        matches!(s, REFERENCE | CLONE | DEREF)
    }
}

impl Default for ReturnType {
    /// Returns the default return type (`Default`).
    ///
    /// # Returns
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
    /// - `s`: The string slice to parse.
    ///
    /// # Returns
    /// - `Result<ReturnType, String>`: Ok containing the `ReturnType` if parsing is successful,
    ///   or an Err containing a String error message if parsing fails.
    #[inline(always)]
    fn from_str(s: &str) -> Result<ReturnType, std::string::String> {
        match s {
            REFERENCE => Ok(ReturnType::Reference),
            CLONE => Ok(ReturnType::Clone),
            DEREF => Ok(ReturnType::Deref),
            _ => Ok(ReturnType::Default),
        }
    }
}
