## lombok-macros

[![](https://img.shields.io/crates/v/lombok-macros.svg)](https://crates.io/crates/lombok-macros)
[![](https://docs.rs/lombok-macros/badge.svg)](https://docs.rs/lombok-macros)
[![](https://img.shields.io/crates/l/lombok-macros.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/lombok-macros/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/lombok-macros/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/lombok-macros/)

[Api Docs](https://docs.rs/lombok-macros/latest/lombok_macros/)

> A collection of procedural macros for Lombok-like functionality in Rust.

## Installation

To use this crate, you can run cmd:

```shell
cargo add lombok-macros
```

## Use

```rust
use lombok_macros::*;

#[derive(Lombok, Debug, Clone)]
struct LombokTest<'a, 'b, T: Clone> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_str_lifetime_a: Option<&'a T>,
    #[set(private)]
    #[get_mut(pub(crate))]
    opt_str_lifetime_b: Option<&'b str>,
}

fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_str_lifetime_a: None,
        opt_str_lifetime_b: None,
    };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    match data.get_list() {
        left_val => {
            assert_eq!(*left_val, list);
        }
    }
    let get_opt_str_lifetime_a: Option<usize> = data.get_opt_str_lifetime_a().cloned();
    assert_eq!(get_opt_str_lifetime_a, None);
    let get_mut_opt_str_lifetime_b: &mut Option<&str> = data.get_mut_opt_str_lifetime_b();
    *get_mut_opt_str_lifetime_b = Some("opt_str_lifetime_b");
    assert_eq!(
        data.get_mut_opt_str_lifetime_b().clone(),
        Some("opt_str_lifetime_b")
    );
}
```

## Macro expansion result

```rs
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use lombok_macros::*;
struct LombokTest<'a, 'b, T: Clone> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_str_lifetime_a: Option<&'a T>,
    #[set(private)]
    #[get_mut(pub(crate))]
    opt_str_lifetime_b: Option<&'b str>,
}
impl<'a, 'b, T: Clone> LombokTest<'a, 'b, T> {
    #[inline]
    pub(crate) fn get_list(&self) -> &Vec<String> {
        &self.list
    }
    #[inline]
    pub(crate) fn set_list(&mut self, val: Vec<String>) -> &mut Self {
        self.list = val;
        self
    }
    #[inline]
    pub fn get_mut_list(&mut self) -> &mut Vec<String> {
        &mut self.list
    }
    #[inline]
    pub(crate) fn get_opt_str_lifetime_a(&self) -> &Option<&'a T> {
        &self.opt_str_lifetime_a
    }
    #[inline]
    pub fn get_mut_opt_str_lifetime_a(&mut self) -> &mut Option<&'a T> {
        &mut self.opt_str_lifetime_a
    }
    #[inline]
    pub fn set_opt_str_lifetime_a(&mut self, val: Option<&'a T>) -> &mut Self {
        self.opt_str_lifetime_a = val;
        self
    }
    #[inline]
    fn set_opt_str_lifetime_b(&mut self, val: Option<&'b str>) -> &mut Self {
        self.opt_str_lifetime_b = val;
        self
    }
    #[inline]
    pub(crate) fn get_mut_opt_str_lifetime_b(&mut self) -> &mut Option<&'b str> {
        &mut self.opt_str_lifetime_b
    }
    #[inline]
    pub fn get_opt_str_lifetime_b(&self) -> &Option<&'b str> {
        &self.opt_str_lifetime_b
    }
}
#[automatically_derived]
impl<'a, 'b, T: ::core::fmt::Debug + Clone> ::core::fmt::Debug
for LombokTest<'a, 'b, T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "LombokTest",
            "list",
            &self.list,
            "opt_str_lifetime_a",
            &self.opt_str_lifetime_a,
            "opt_str_lifetime_b",
            &&self.opt_str_lifetime_b,
        )
    }
}
#[automatically_derived]
impl<'a, 'b, T: ::core::clone::Clone + Clone> ::core::clone::Clone
for LombokTest<'a, 'b, T> {
    #[inline]
    fn clone(&self) -> LombokTest<'a, 'b, T> {
        LombokTest {
            list: ::core::clone::Clone::clone(&self.list),
            opt_str_lifetime_a: ::core::clone::Clone::clone(&self.opt_str_lifetime_a),
            opt_str_lifetime_b: ::core::clone::Clone::clone(&self.opt_str_lifetime_b),
        }
    }
}
fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_str_lifetime_a: None,
        opt_str_lifetime_b: None,
    };
    let list: Vec<String> = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new(["hello".to_string(), "world".to_string()]),
    );
    data.set_list(list.clone());
    match data.get_list() {
        left_val => {
            match (&*left_val, &list) {
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
        }
    }
    let get_opt_str_lifetime_a: Option<usize> = data.get_opt_str_lifetime_a().cloned();
    match (&get_opt_str_lifetime_a, &None) {
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
    let get_mut_opt_str_lifetime_b: &mut Option<&str> = data
        .get_mut_opt_str_lifetime_b();
    *get_mut_opt_str_lifetime_b = Some("opt_str_lifetime_b");
    match (&data.get_mut_opt_str_lifetime_b().clone(), &Some("opt_str_lifetime_b")) {
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
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
