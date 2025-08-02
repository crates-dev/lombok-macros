<center>

## lombok-macros

[![](https://img.shields.io/crates/v/lombok-macros.svg)](https://crates.io/crates/lombok-macros)
[![](https://img.shields.io/crates/d/lombok-macros.svg)](https://img.shields.io/crates/d/lombok-macros.svg)
[![](https://docs.rs/lombok-macros/badge.svg)](https://docs.rs/lombok-macros)
[![](https://github.com/crates-dev/lombok-macros/workflows/Rust/badge.svg)](https://github.com/crates-dev/lombok-macros/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/lombok-macros.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/lombok-macros/)

[Api Docs](https://docs.rs/lombok-macros/latest/lombok_macros/)

> A Rust procedural macro collection providing Lombok-like functionality. Automatically generates getters/setters with field-level visibility control, custom Debug implementations with field skipping, and Display trait implementations. Supports structs, enums, generics and lifetimes.

## Installation

To use this crate, you can run cmd:

```shell
cargo add lombok-macros
```

## Use

### Code

```rust
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
```

### Macro expansion result

```rust
#![feature(prelude_import)]
#![allow(warnings)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use lombok_macros::*;
use std::fmt::Debug;
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
impl<'a, T: Clone + Debug> LombokTest<'a, T> {
    pub(crate) fn get_list(&self) -> &Vec<String> {
        &self.list
    }
    pub(crate) fn set_list(&mut self, val: Vec<String>) -> &mut Self {
        self.list = val;
        self
    }
    pub fn get_mut_list(&mut self) -> &mut Vec<String> {
        &mut self.list
    }
    pub(crate) fn get_opt_value(&self) -> &Option<&'a T> {
        &self.opt_value
    }
    pub fn get_mut_opt_value(&mut self) -> &mut Option<&'a T> {
        &mut self.opt_value
    }
    pub fn set_opt_value(&mut self, val: Option<&'a T>) -> &mut Self {
        self.opt_value = val;
        self
    }
    pub(crate) fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    fn set_name(&mut self, val: String) -> &mut Self {
        self.name = val;
        self
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
#[automatically_derived]
impl<'a, T: ::core::fmt::Debug + Clone + Debug> ::core::fmt::Debug
for LombokTest<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "LombokTest",
            "list",
            &self.list,
            "opt_value",
            &self.opt_value,
            "name",
            &&self.name,
        )
    }
}
#[automatically_derived]
impl<'a, T: ::core::clone::Clone + Clone + Debug> ::core::clone::Clone
for LombokTest<'a, T> {
    #[inline]
    fn clone(&self) -> LombokTest<'a, T> {
        LombokTest {
            list: ::core::clone::Clone::clone(&self.list),
            opt_value: ::core::clone::Clone::clone(&self.opt_value),
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
impl<'a, T: Clone + Debug> std::fmt::Display for LombokTest<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0:#?}", self))
    }
}
struct User {
    name: String,
    #[debug(skip)]
    password: String,
    email: String,
}
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .finish()
    }
}
struct TupleStruct(#[get(pub)] String, #[set(pub)] i32, #[get(pub)] #[set(pub)] bool);
impl TupleStruct {
    pub fn get_0(&self) -> &String {
        &self.0
    }
    pub fn get_mut_0(&mut self) -> &mut String {
        &mut self.0
    }
    pub fn set_0(&mut self, val: String) -> &mut Self {
        self.0 = val;
        self
    }
    pub fn set_1(&mut self, val: i32) -> &mut Self {
        self.1 = val;
        self
    }
    pub fn get_1(&self) -> &i32 {
        &self.1
    }
    pub fn get_mut_1(&mut self) -> &mut i32 {
        &mut self.1
    }
    pub fn get_2(&self) -> &bool {
        &self.2
    }
    pub fn set_2(&mut self, val: bool) -> &mut Self {
        self.2 = val;
        self
    }
    pub fn get_mut_2(&mut self) -> &mut bool {
        &mut self.2
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TupleStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(
            f,
            "TupleStruct",
            &self.0,
            &self.1,
            &&self.2,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TupleStruct {
    #[inline]
    fn clone(&self) -> TupleStruct {
        TupleStruct(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
            ::core::clone::Clone::clone(&self.2),
        )
    }
}
enum Response {
    Success { data: String },
    Error { message: String, #[debug(skip)] internal_code: u32 },
}
impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Success { data } => {
                f.debug_struct("Success").field("data", data).finish()
            }
            Response::Error { message, internal_code } => {
                f.debug_struct("Error").field("message", message).finish()
            }
        }
    }
}
fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        name: "test".to_string(),
    };
    let list: Vec<String> = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new(["hello".to_string(), "world".to_string()]),
    );
    data.set_list(list.clone());
    match (&*data.get_list(), &list) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let opt_value: &Option<&usize> = data.get_opt_value();
    match (&*opt_value, &None) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let name_mut: &mut String = data.get_mut_name();
    *name_mut = "updated".to_string();
    if !!data.to_string().is_empty() {
        ::core::panicking::panic("assertion failed: !data.to_string().is_empty()")
    }
    let mut tuple_data: TupleStruct = TupleStruct("hello".to_string(), 42, true);
    let field0: &String = tuple_data.get_0();
    match (&field0, &"hello") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    tuple_data.set_1(100);
    let field2: &bool = tuple_data.get_2();
    match (&*field2, &true) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    tuple_data.set_2(false);
    let user: User = User {
        name: "Alice".to_string(),
        password: "secret123".to_string(),
        email: "alice@example.com".to_string(),
    };
    let user_debug: String = ::alloc::__export::must_use({
        let res = ::alloc::fmt::format(format_args!("{0:?}", user));
        res
    });
    if !user_debug.contains("Alice") {
        ::core::panicking::panic("assertion failed: user_debug.contains(\"Alice\")")
    }
    if !user_debug.contains("alice@example.com") {
        ::core::panicking::panic(
            "assertion failed: user_debug.contains(\"alice@example.com\")",
        )
    }
    if !!user_debug.contains("secret123") {
        ::core::panicking::panic("assertion failed: !user_debug.contains(\"secret123\")")
    }
    let success: Response = Response::Success {
        data: "Operation completed".to_string(),
    };
    let success_debug: String = ::alloc::__export::must_use({
        let res = ::alloc::fmt::format(format_args!("{0:?}", success));
        res
    });
    if !success_debug.contains("Operation completed") {
        ::core::panicking::panic(
            "assertion failed: success_debug.contains(\"Operation completed\")",
        )
    }
    let error: Response = Response::Error {
        message: "Something went wrong".to_string(),
        internal_code: 500,
    };
    let error_debug: String = ::alloc::__export::must_use({
        let res = ::alloc::fmt::format(format_args!("{0:?}", error));
        res
    });
    if !error_debug.contains("Something went wrong") {
        ::core::panicking::panic(
            "assertion failed: error_debug.contains(\"Something went wrong\")",
        )
    }
    if !!error_debug.contains("500") {
        ::core::panicking::panic("assertion failed: !error_debug.contains(\"500\")")
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
