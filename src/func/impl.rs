use crate::*;

impl Default for FuncType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for FuncType {
    type Err = String;

    fn from_str(s: &str) -> Result<FuncType, std::string::String> {
        match s {
            GET => Ok(FuncType::Get),
            GET_MUT => Ok(FuncType::GetMut),
            SET => Ok(FuncType::Set),
            _ => Ok(FuncType::Unknown),
        }
    }
}

impl FuncType {
    /// Checks if the `FuncType` is `Get`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Get`; otherwise, `false`.
    pub fn is_get(&self) -> bool {
        *self == FuncType::Get
    }

    /// Checks if the `FuncType` is `GetMut`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `GetMut`; otherwise, `false`.
    pub fn is_get_mut(&self) -> bool {
        *self == FuncType::GetMut
    }

    /// Checks if the `FuncType` is `Set`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Set`; otherwise, `false`.
    pub fn is_set(&self) -> bool {
        *self == FuncType::Set
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Unknown`; otherwise, `false`.
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    /// Checks if the `FuncType` is `Unknown`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Unknown`; otherwise, `false`.
    pub fn is_known(func_type_str: &str) -> bool {
        let func_type: FuncType = func_type_str.parse::<FuncType>().unwrap_or_default();
        func_type != Self::Unknown
    }
}
