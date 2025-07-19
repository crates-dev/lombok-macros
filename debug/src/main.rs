#![allow(warnings)]

use lombok_macros::*;
use std::fmt::Debug;

#[derive(Data, Debug, Clone, DisplayDebugFormat)]
struct LombokTest<'a, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_value: Option<&'a T>,
    #[get_mut(pub(crate))]
    #[set(private)]
    name: String,
}

#[derive(CustomDebug)]
struct User {
    name: String,
    #[debug(skip)]
    password: String,
    email: String,
}

#[derive(Data, Debug, Clone)]
struct TupleStruct(
    #[get(pub)] String,
    #[set(pub)] i32,
    #[get(pub)]
    #[set(pub)]
    bool,
);

#[derive(CustomDebug)]
enum Response {
    Success {
        data: String,
    },
    Error {
        message: String,
        #[debug(skip)]
        internal_code: u32,
    },
}

fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        name: "test".to_string(),
    };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    assert_eq!(*data.get_list(), list);
    let opt_value: &Option<&usize> = data.get_opt_value();
    assert_eq!(*opt_value, None);
    let name_mut: &mut String = data.get_mut_name();
    *name_mut = "updated".to_string();
    assert!(!data.to_string().is_empty());
    let mut tuple_data: TupleStruct = TupleStruct("hello".to_string(), 42, true);
    let field0: &String = tuple_data.get_0();
    assert_eq!(field0, "hello");
    tuple_data.set_1(100);
    let field2: &bool = tuple_data.get_2();
    assert_eq!(*field2, true);
    tuple_data.set_2(false);
    let user: User = User {
        name: "Alice".to_string(),
        password: "secret123".to_string(),
        email: "alice@example.com".to_string(),
    };
    let user_debug: String = format!("{:?}", user);
    assert!(user_debug.contains("Alice"));
    assert!(user_debug.contains("alice@example.com"));
    assert!(!user_debug.contains("secret123"));
    let success: Response = Response::Success {
        data: "Operation completed".to_string(),
    };
    let success_debug: String = format!("{:?}", success);
    assert!(success_debug.contains("Operation completed"));
    let error: Response = Response::Error {
        message: "Something went wrong".to_string(),
        internal_code: 500,
    };
    let error_debug: String = format!("{:?}", error);
    assert!(error_debug.contains("Something went wrong"));
    assert!(!error_debug.contains("500"));
}
