#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use lombok_macros::*;
use std::{f64::consts::PI, fmt::Debug};
struct MultiAttributes {
    #[get(pub, type(clone))]
    #[set(pub(super), type(Into<Vec<String>>))]
    complex_field: Vec<String>,
}
impl MultiAttributes {
    #[inline(always)]
    pub fn get_complex_field(&self) -> Vec<String> {
        self.complex_field.clone()
    }
    #[inline(always)]
    pub(super) fn set_complex_field(
        &mut self,
        val: impl Into<Vec<String>>,
    ) -> &mut Self {
        self.complex_field = val.into();
        self
    }
    #[inline(always)]
    pub fn get_mut_complex_field(&mut self) -> &mut Vec<String> {
        &mut self.complex_field
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MultiAttributes {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "MultiAttributes",
            "complex_field",
            &&self.complex_field,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MultiAttributes {
    #[inline]
    fn clone(&self) -> MultiAttributes {
        MultiAttributes {
            complex_field: ::core::clone::Clone::clone(&self.complex_field),
        }
    }
}
fn main() {
    let multi = MultiAttributes {
        complex_field: <[_]>::into_vec(::alloc::boxed::box_new(["test".to_string()])),
    };
    let cloned_field = multi.get_complex_field();
    match (
        &cloned_field,
        &<[_]>::into_vec(::alloc::boxed::box_new(["test".to_string()])),
    ) {
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
    let mut mutated = multi;
    let new_vec = <[_]>::into_vec(
        ::alloc::boxed::box_new(["new".to_string(), "values".to_string()]),
    );
    mutated.set_complex_field(new_vec.clone());
    let updated = mutated.get_complex_field();
    match (&updated, &new_vec) {
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
