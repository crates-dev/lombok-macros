use super::r#type::Cfg;
use crate::{func::r#type::FuncType, visibility::r#type::Visibility};

impl Default for Cfg {
    #[inline]
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknow,
            skip: false,
            visibility: Visibility::Public,
        }
    }
}
