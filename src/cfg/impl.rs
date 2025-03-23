use crate::*;

impl Default for Cfg {
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknow,
            skip: false,
            visibility: Visibility::Public,
        }
    }
}
