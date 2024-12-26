use super::r#type::FuncType;
use crate::parse::constant::{GET, SET};
use std::str::FromStr;

impl Default for FuncType {
    fn default() -> Self {
        Self::Unknow
    }
}

impl FromStr for FuncType {
    type Err = String;

    fn from_str(s: &str) -> Result<FuncType, std::string::String> {
        match s {
            GET => Ok(FuncType::Get),
            SET => Ok(FuncType::Set),
            _ => Ok(FuncType::Unknow),
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

    /// Checks if the `FuncType` is `Unknow`.
    ///
    /// # Parameters
    /// - `self`: The reference to the `FuncType` instance.
    ///
    /// # Returns
    /// - `true` if the `FuncType` is `Unknow`; otherwise, `false`.
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknow
    }
}
