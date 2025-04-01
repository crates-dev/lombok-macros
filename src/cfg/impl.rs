use crate::*;

impl Default for Cfg {
    fn default() -> Self {
        Self {
            func_type: FuncType::Unknown,
            skip: false,
            visibility: Visibility::Public,
        }
    }
}
