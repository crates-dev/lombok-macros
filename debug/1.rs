#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use lombok_macros::*;
use std::{f64::consts::PI, fmt::Debug};
struct LombokTest<'a, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_value: Option<&'a T>,
    #[get(pub(crate))]
    result_value: Result<&'a T, &'static str>,
    #[get_mut(pub(crate))]
    #[set(private)]
    name: String,
}
impl<'a, T: Clone + Debug> LombokTest<'a, T> {
    #[inline(always)]
    pub(crate) fn get_list(&self) -> &Vec<String> {
        &self.list
    }
    #[inline(always)]
    pub(crate) fn set_list(&mut self, val: Vec<String>) -> &mut Self {
        self.list = val;
        self
    }
    #[inline(always)]
    pub fn get_mut_list(&mut self) -> &mut Vec<String> {
        &mut self.list
    }
    #[inline(always)]
    pub(crate) fn get_opt_value(&self) -> &'a T {
        self.opt_value.clone().unwrap()
    }
    #[inline(always)]
    pub(crate) fn try_get_opt_value(&self) -> &Option<&'a T> {
        &self.opt_value
    }
    #[inline(always)]
    pub fn get_mut_opt_value(&mut self) -> &mut Option<&'a T> {
        &mut self.opt_value
    }
    #[inline(always)]
    pub fn set_opt_value(&mut self, val: Option<&'a T>) -> &mut Self {
        self.opt_value = val;
        self
    }
    #[inline(always)]
    pub(crate) fn get_result_value(&self) -> &'a T {
        self.result_value.clone().unwrap()
    }
    #[inline(always)]
    pub(crate) fn try_get_result_value(&self) -> &Result<&'a T, &'static str> {
        &self.result_value
    }
    #[inline(always)]
    pub fn get_mut_result_value(&mut self) -> &mut Result<&'a T, &'static str> {
        &mut self.result_value
    }
    #[inline(always)]
    pub fn set_result_value(&mut self, val: Result<&'a T, &'static str>) -> &mut Self {
        self.result_value = val;
        self
    }
    #[inline(always)]
    pub(crate) fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    #[inline(always)]
    fn set_name(&mut self, val: String) -> &mut Self {
        self.name = val;
        self
    }
    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
#[automatically_derived]
impl<'a, T: ::core::fmt::Debug + Clone + Debug> ::core::fmt::Debug
for LombokTest<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "LombokTest",
            "list",
            &self.list,
            "opt_value",
            &self.opt_value,
            "result_value",
            &self.result_value,
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
            result_value: ::core::clone::Clone::clone(&self.result_value),
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
    _password: String,
    #[new(skip)]
    email: Option<String>,
}
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .finish()
    }
}
impl User {
    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
    #[inline(always)]
    pub fn get__password(&self) -> &String {
        &self._password
    }
    #[inline(always)]
    pub fn get_email(&self) -> String {
        self.email.clone().unwrap()
    }
    #[inline(always)]
    pub fn try_get_email(&self) -> &Option<String> {
        &self.email
    }
}
impl User {
    #[inline(always)]
    pub fn new(name: String, _password: String) -> Self {
        Self {
            name: name,
            _password: _password,
            email: Default::default(),
        }
    }
}
struct TupleStruct(#[get(pub)] String, #[set(pub)] i32, #[get(pub)] #[set(pub)] bool);
impl TupleStruct {
    #[inline(always)]
    pub fn get_0(&self) -> &String {
        &self.0
    }
    #[inline(always)]
    pub fn get_mut_0(&mut self) -> &mut String {
        &mut self.0
    }
    #[inline(always)]
    pub fn set_0(&mut self, val: String) -> &mut Self {
        self.0 = val;
        self
    }
    #[inline(always)]
    pub fn set_1(&mut self, val: i32) -> &mut Self {
        self.1 = val;
        self
    }
    #[inline(always)]
    pub fn get_1(&self) -> &i32 {
        &self.1
    }
    #[inline(always)]
    pub fn get_mut_1(&mut self) -> &mut i32 {
        &mut self.1
    }
    #[inline(always)]
    pub fn get_2(&self) -> &bool {
        &self.2
    }
    #[inline(always)]
    pub fn set_2(&mut self, val: bool) -> &mut Self {
        self.2 = val;
        self
    }
    #[inline(always)]
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
struct TraitTestStruct {
    #[set(pub, type(AsRef<str>))]
    name: String,
    #[get(type(clone))]
    #[set(pub, type(Into<i32>))]
    value: i32,
    #[set(pub, type(AsRef<[u8]>))]
    data: Vec<u8>,
    #[set(pub, type(Into<Vec<String>>))]
    items: Vec<String>,
}
impl TraitTestStruct {
    #[inline(always)]
    pub fn set_name(&mut self, val: impl AsRef<str>) -> &mut Self {
        self.name = val.as_ref().to_owned();
        self
    }
    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
    #[inline(always)]
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    #[inline(always)]
    pub fn get_value(&self) -> i32 {
        self.value.clone()
    }
    #[inline(always)]
    pub fn set_value(&mut self, val: impl Into<i32>) -> &mut Self {
        self.value = val.into();
        self
    }
    #[inline(always)]
    pub fn get_mut_value(&mut self) -> &mut i32 {
        &mut self.value
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: impl AsRef<[u8]>) -> &mut Self {
        self.data = val.as_ref().to_owned();
        self
    }
    #[inline(always)]
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    #[inline(always)]
    pub fn get_mut_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    #[inline(always)]
    pub fn set_items(&mut self, val: impl Into<Vec<String>>) -> &mut Self {
        self.items = val.into();
        self
    }
    #[inline(always)]
    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }
    #[inline(always)]
    pub fn get_mut_items(&mut self) -> &mut Vec<String> {
        &mut self.items
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TraitTestStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "TraitTestStruct",
            "name",
            &self.name,
            "value",
            &self.value,
            "data",
            &self.data,
            "items",
            &&self.items,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TraitTestStruct {
    #[inline]
    fn clone(&self) -> TraitTestStruct {
        TraitTestStruct {
            name: ::core::clone::Clone::clone(&self.name),
            value: ::core::clone::Clone::clone(&self.value),
            data: ::core::clone::Clone::clone(&self.data),
            items: ::core::clone::Clone::clone(&self.items),
        }
    }
}
struct TupleWithResult(
    #[get(pub, type(clone))]
    String,
    #[get(pub)]
    Result<i32, &'static str>,
);
impl TupleWithResult {
    #[inline(always)]
    pub fn get_0(&self) -> String {
        self.0.clone()
    }
    #[inline(always)]
    pub fn get_mut_0(&mut self) -> &mut String {
        &mut self.0
    }
    #[inline(always)]
    pub fn set_0(&mut self, val: String) -> &mut Self {
        self.0 = val;
        self
    }
    #[inline(always)]
    pub fn get_1(&self) -> i32 {
        self.1.clone().unwrap()
    }
    #[inline(always)]
    pub fn try_get_1(&self) -> &Result<i32, &'static str> {
        &self.1
    }
    #[inline(always)]
    pub fn get_mut_1(&mut self) -> &mut Result<i32, &'static str> {
        &mut self.1
    }
    #[inline(always)]
    pub fn set_1(&mut self, val: Result<i32, &'static str>) -> &mut Self {
        self.1 = val;
        self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TupleWithResult {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(
            f,
            "TupleWithResult",
            &self.0,
            &&self.1,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TupleWithResult {
    #[inline]
    fn clone(&self) -> TupleWithResult {
        TupleWithResult(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
enum Response {
    Success { data: String },
    Error { message: String, #[debug(skip)] _internal_code: u32 },
}
impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Success { data } => {
                f.debug_struct("Success").field("data", data).finish()
            }
            Response::Error { message, _internal_code } => {
                f.debug_struct("Error").field("message", message).finish()
            }
        }
    }
}
struct Person {
    name: String,
    _age: u32,
}
impl Person {
    #[inline(always)]
    pub fn new(name: String, _age: u32) -> Self {
        Self { name: name, _age: _age }
    }
}
#[new(pub)]
struct PublicPerson {
    _name: String,
    _age: u32,
}
impl PublicPerson {
    #[inline(always)]
    pub fn new(_name: String, _age: u32) -> Self {
        Self { _name: _name, _age: _age }
    }
}
#[new(pub(crate))]
struct CratePerson {
    _name: String,
    _age: u32,
}
impl CratePerson {
    #[inline(always)]
    pub(crate) fn new(_name: String, _age: u32) -> Self {
        Self { _name: _name, _age: _age }
    }
}
#[new(private)]
struct PrivatePerson {
    _name: String,
    _age: u32,
}
impl PrivatePerson {
    #[inline(always)]
    fn new(_name: String, _age: u32) -> Self {
        Self { _name: _name, _age: _age }
    }
}
#[new(private)]
struct Product {
    id: u64,
    name: String,
    _price: f64,
    #[new(skip)]
    _description: String,
}
impl Product {
    #[inline(always)]
    pub fn get_id(&self) -> &u64 {
        &self.id
    }
    #[inline(always)]
    pub fn get_mut_id(&mut self) -> &mut u64 {
        &mut self.id
    }
    #[inline(always)]
    pub fn set_id(&mut self, val: u64) -> &mut Self {
        self.id = val;
        self
    }
    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
    #[inline(always)]
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    #[inline(always)]
    pub fn set_name(&mut self, val: String) -> &mut Self {
        self.name = val;
        self
    }
    #[inline(always)]
    pub fn get__price(&self) -> &f64 {
        &self._price
    }
    #[inline(always)]
    pub fn get_mut__price(&mut self) -> &mut f64 {
        &mut self._price
    }
    #[inline(always)]
    pub fn set__price(&mut self, val: f64) -> &mut Self {
        self._price = val;
        self
    }
    #[inline(always)]
    pub fn get__description(&self) -> &String {
        &self._description
    }
    #[inline(always)]
    pub fn get_mut__description(&mut self) -> &mut String {
        &mut self._description
    }
    #[inline(always)]
    pub fn set__description(&mut self, val: String) -> &mut Self {
        self._description = val;
        self
    }
}
impl Product {
    #[inline(always)]
    fn new(id: u64, name: String, _price: f64) -> Self {
        Self {
            id: id,
            name: name,
            _price: _price,
            _description: Default::default(),
        }
    }
}
struct TuplePoint(f64, #[new(skip)] f64, f64);
impl TuplePoint {
    #[inline(always)]
    pub fn new(field_0: f64, field_2: f64) -> Self {
        Self(field_0, Default::default(), field_2)
    }
}
struct NestedStruct {
    #[get(pub)]
    name: String,
    #[set(pub)]
    _value: i32,
}
impl NestedStruct {
    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
    #[inline(always)]
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    #[inline(always)]
    pub fn set_name(&mut self, val: String) -> &mut Self {
        self.name = val;
        self
    }
    #[inline(always)]
    pub fn set__value(&mut self, val: i32) -> &mut Self {
        self._value = val;
        self
    }
    #[inline(always)]
    pub fn get__value(&self) -> &i32 {
        &self._value
    }
    #[inline(always)]
    pub fn get_mut__value(&mut self) -> &mut i32 {
        &mut self._value
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for NestedStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "NestedStruct",
            "name",
            &self.name,
            "_value",
            &&self._value,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for NestedStruct {
    #[inline]
    fn clone(&self) -> NestedStruct {
        NestedStruct {
            name: ::core::clone::Clone::clone(&self.name),
            _value: ::core::clone::Clone::clone(&self._value),
        }
    }
}
struct ComplexNestedStruct {
    #[get(pub)]
    nested: NestedStruct,
    #[get(pub)]
    nested_list: Vec<NestedStruct>,
    #[set(pub)]
    metadata: std::collections::HashMap<String, String>,
}
impl ComplexNestedStruct {
    #[inline(always)]
    pub fn get_nested(&self) -> &NestedStruct {
        &self.nested
    }
    #[inline(always)]
    pub fn get_mut_nested(&mut self) -> &mut NestedStruct {
        &mut self.nested
    }
    #[inline(always)]
    pub fn set_nested(&mut self, val: NestedStruct) -> &mut Self {
        self.nested = val;
        self
    }
    #[inline(always)]
    pub fn get_nested_list(&self) -> &Vec<NestedStruct> {
        &self.nested_list
    }
    #[inline(always)]
    pub fn get_mut_nested_list(&mut self) -> &mut Vec<NestedStruct> {
        &mut self.nested_list
    }
    #[inline(always)]
    pub fn set_nested_list(&mut self, val: Vec<NestedStruct>) -> &mut Self {
        self.nested_list = val;
        self
    }
    #[inline(always)]
    pub fn set_metadata(
        &mut self,
        val: std::collections::HashMap<String, String>,
    ) -> &mut Self {
        self.metadata = val;
        self
    }
    #[inline(always)]
    pub fn get_metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }
    #[inline(always)]
    pub fn get_mut_metadata(
        &mut self,
    ) -> &mut std::collections::HashMap<String, String> {
        &mut self.metadata
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ComplexNestedStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "ComplexNestedStruct",
            "nested",
            &self.nested,
            "nested_list",
            &self.nested_list,
            "metadata",
            &&self.metadata,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ComplexNestedStruct {
    #[inline]
    fn clone(&self) -> ComplexNestedStruct {
        ComplexNestedStruct {
            nested: ::core::clone::Clone::clone(&self.nested),
            nested_list: ::core::clone::Clone::clone(&self.nested_list),
            metadata: ::core::clone::Clone::clone(&self.metadata),
        }
    }
}
enum ComplexEnum {
    Simple,
    Tuple(String, i32),
    Struct { field1: String, #[debug(skip)] _secret: String, value: f64 },
}
impl std::fmt::Debug for ComplexEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexEnum::Simple => f.debug_struct("Simple").finish(),
            ComplexEnum::Tuple(field_0, field_1) => {
                f.debug_tuple("Tuple").field(field_0).field(field_1).finish()
            }
            ComplexEnum::Struct { field1, _secret, value } => {
                f.debug_struct("Struct")
                    .field("field1", field1)
                    .field("value", value)
                    .finish()
            }
        }
    }
}
struct GenericStruct<T: Default + Clone> {
    #[new(skip)]
    data: T,
    value: i32,
}
impl<T: Default + Clone> GenericStruct<T> {
    #[inline(always)]
    pub fn new(value: i32) -> Self {
        Self {
            data: Default::default(),
            value: value,
        }
    }
}
struct LifetimesTest<'a, 'b> {
    #[get(pub)]
    name: &'a str,
    #[get(pub)]
    description: &'b str,
}
impl<'a, 'b> LifetimesTest<'a, 'b> {
    #[inline(always)]
    pub fn get_name(&self) -> &&'a str {
        &self.name
    }
    #[inline(always)]
    pub fn get_mut_name(&mut self) -> &mut &'a str {
        &mut self.name
    }
    #[inline(always)]
    pub fn set_name(&mut self, val: &'a str) -> &mut Self {
        self.name = val;
        self
    }
    #[inline(always)]
    pub fn get_description(&self) -> &&'b str {
        &self.description
    }
    #[inline(always)]
    pub fn get_mut_description(&mut self) -> &mut &'b str {
        &mut self.description
    }
    #[inline(always)]
    pub fn set_description(&mut self, val: &'b str) -> &mut Self {
        self.description = val;
        self
    }
}
#[automatically_derived]
impl<'a, 'b> ::core::fmt::Debug for LifetimesTest<'a, 'b> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "LifetimesTest",
            "name",
            &self.name,
            "description",
            &&self.description,
        )
    }
}
#[automatically_derived]
impl<'a, 'b> ::core::clone::Clone for LifetimesTest<'a, 'b> {
    #[inline]
    fn clone(&self) -> LifetimesTest<'a, 'b> {
        LifetimesTest {
            name: ::core::clone::Clone::clone(&self.name),
            description: ::core::clone::Clone::clone(&self.description),
        }
    }
}
struct EdgeCaseTest {
    #[get(pub)]
    empty_string: String,
    #[get(pub, type(clone))]
    empty_vec: Vec<i32>,
    #[get(pub, type(clone))]
    zero_value: i32,
    #[get(pub, type(clone))]
    bool_false: bool,
    #[get(pub)]
    option_none: Option<String>,
}
impl EdgeCaseTest {
    #[inline(always)]
    pub fn get_empty_string(&self) -> &String {
        &self.empty_string
    }
    #[inline(always)]
    pub fn get_mut_empty_string(&mut self) -> &mut String {
        &mut self.empty_string
    }
    #[inline(always)]
    pub fn set_empty_string(&mut self, val: String) -> &mut Self {
        self.empty_string = val;
        self
    }
    #[inline(always)]
    pub fn get_empty_vec(&self) -> Vec<i32> {
        self.empty_vec.clone()
    }
    #[inline(always)]
    pub fn get_mut_empty_vec(&mut self) -> &mut Vec<i32> {
        &mut self.empty_vec
    }
    #[inline(always)]
    pub fn set_empty_vec(&mut self, val: Vec<i32>) -> &mut Self {
        self.empty_vec = val;
        self
    }
    #[inline(always)]
    pub fn get_zero_value(&self) -> i32 {
        self.zero_value.clone()
    }
    #[inline(always)]
    pub fn get_mut_zero_value(&mut self) -> &mut i32 {
        &mut self.zero_value
    }
    #[inline(always)]
    pub fn set_zero_value(&mut self, val: i32) -> &mut Self {
        self.zero_value = val;
        self
    }
    #[inline(always)]
    pub fn get_bool_false(&self) -> bool {
        self.bool_false.clone()
    }
    #[inline(always)]
    pub fn get_mut_bool_false(&mut self) -> &mut bool {
        &mut self.bool_false
    }
    #[inline(always)]
    pub fn set_bool_false(&mut self, val: bool) -> &mut Self {
        self.bool_false = val;
        self
    }
    #[inline(always)]
    pub fn get_option_none(&self) -> String {
        self.option_none.clone().unwrap()
    }
    #[inline(always)]
    pub fn try_get_option_none(&self) -> &Option<String> {
        &self.option_none
    }
    #[inline(always)]
    pub fn get_mut_option_none(&mut self) -> &mut Option<String> {
        &mut self.option_none
    }
    #[inline(always)]
    pub fn set_option_none(&mut self, val: Option<String>) -> &mut Self {
        self.option_none = val;
        self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EdgeCaseTest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "EdgeCaseTest",
            "empty_string",
            &self.empty_string,
            "empty_vec",
            &self.empty_vec,
            "zero_value",
            &self.zero_value,
            "bool_false",
            &self.bool_false,
            "option_none",
            &&self.option_none,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EdgeCaseTest {
    #[inline]
    fn clone(&self) -> EdgeCaseTest {
        EdgeCaseTest {
            empty_string: ::core::clone::Clone::clone(&self.empty_string),
            empty_vec: ::core::clone::Clone::clone(&self.empty_vec),
            zero_value: ::core::clone::Clone::clone(&self.zero_value),
            bool_false: ::core::clone::Clone::clone(&self.bool_false),
            option_none: ::core::clone::Clone::clone(&self.option_none),
        }
    }
}
struct CopyTest {
    #[get(skip, pub, type(copy))]
    value: i32,
    #[get(pub(super))]
    flag: bool,
    #[get(private, type(copy))]
    count: u64,
}
impl CopyTest {
    #[inline(always)]
    pub fn get_value(&self) -> &i32 {
        &self.value
    }
    #[inline(always)]
    pub fn get_mut_value(&mut self) -> &mut i32 {
        &mut self.value
    }
    #[inline(always)]
    pub fn set_value(&mut self, val: i32) -> &mut Self {
        self.value = val;
        self
    }
    #[inline(always)]
    pub(super) fn get_flag(&self) -> &bool {
        &self.flag
    }
    #[inline(always)]
    pub fn get_mut_flag(&mut self) -> &mut bool {
        &mut self.flag
    }
    #[inline(always)]
    pub fn set_flag(&mut self, val: bool) -> &mut Self {
        self.flag = val;
        self
    }
    #[inline(always)]
    fn get_count(&self) -> u64 {
        self.count
    }
    #[inline(always)]
    pub fn get_mut_count(&mut self) -> &mut u64 {
        &mut self.count
    }
    #[inline(always)]
    pub fn set_count(&mut self, val: u64) -> &mut Self {
        self.count = val;
        self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CopyTest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CopyTest",
            "value",
            &self.value,
            "flag",
            &self.flag,
            "count",
            &&self.count,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CopyTest {
    #[inline]
    fn clone(&self) -> CopyTest {
        CopyTest {
            value: ::core::clone::Clone::clone(&self.value),
            flag: ::core::clone::Clone::clone(&self.flag),
            count: ::core::clone::Clone::clone(&self.count),
        }
    }
}
struct UnitGetSet {
    #[get(pub)]
    flag: bool,
}
impl UnitGetSet {
    #[inline(always)]
    pub fn get_flag(&self) -> &bool {
        &self.flag
    }
    #[inline(always)]
    pub fn get_mut_flag(&mut self) -> &mut bool {
        &mut self.flag
    }
    #[inline(always)]
    pub fn set_flag(&mut self, val: bool) -> &mut Self {
        self.flag = val;
        self
    }
}
struct AllSkipped {
    #[new(skip)]
    skipped1: String,
    #[new(skip)]
    skipped2: i32,
}
impl AllSkipped {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            skipped1: Default::default(),
            skipped2: Default::default(),
        }
    }
}
struct MultiAttributes {
    #[get(pub, type(clone))]
    #[set(pub(super))]
    complex_field: Vec<String>,
}
impl MultiAttributes {
    #[inline(always)]
    pub fn get_complex_field(&self) -> Vec<String> {
        self.complex_field.clone()
    }
    #[inline(always)]
    pub(super) fn set_complex_field(&mut self, val: Vec<String>) -> &mut Self {
        self.complex_field = val;
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
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        result_value: Err("error"),
        name: "test".to_string(),
    };
    let list: Vec<String> = <[_]>::into_vec(
        ::alloc::boxed::box_new(["hello".to_string(), "world".to_string()]),
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
    let opt_value: &Option<&usize> = data.try_get_opt_value();
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
    data.set_opt_value(Some(&42));
    let try_opt_value: &Option<&usize> = data.try_get_opt_value();
    match (&try_opt_value, &&Some(&42)) {
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
    let unwrap_value: &usize = data.get_opt_value();
    match (&unwrap_value, &&42) {
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
    let result_value: &Result<&usize, &str> = data.try_get_result_value();
    match (&*result_value, &Err("error")) {
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
    data.set_result_value(Ok(&100));
    let try_result_value: &Result<&usize, &str> = data.try_get_result_value();
    match (&try_result_value, &&Ok(&100)) {
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
    let unwrap_result: &usize = data.get_result_value();
    match (&unwrap_result, &&100) {
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
    if !*field2 {
        ::core::panicking::panic("assertion failed: *field2")
    }
    tuple_data.set_2(false);
    let mut tuple_result = TupleWithResult("test".to_string(), Err("error"));
    let try_result: String = tuple_result.get_0();
    match (&try_result, &String::from("test")) {
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
    let try_result: &Result<i32, &str> = tuple_result.try_get_1();
    match (&*try_result, &Err("error")) {
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
    tuple_result.1 = Ok(42);
    let unwrap_result: i32 = tuple_result.get_1();
    match (&unwrap_result, &42) {
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
    let user: User = User {
        name: "Alice".to_string(),
        _password: "secret123".to_string(),
        email: Some("alice@ltpp.vip".to_string()),
    };
    match (&user.get_name(), &"Alice") {
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
    match (&user.get_email(), &"alice@ltpp.vip".to_string()) {
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
    let user_debug: String = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", user))
    });
    if !user_debug.contains("Alice") {
        ::core::panicking::panic("assertion failed: user_debug.contains(\"Alice\")")
    }
    if !user_debug.contains("alice@ltpp.vip") {
        ::core::panicking::panic(
            "assertion failed: user_debug.contains(\"alice@ltpp.vip\")",
        )
    }
    if !!user_debug.contains("secret123") {
        ::core::panicking::panic("assertion failed: !user_debug.contains(\"secret123\")")
    }
    let success: Response = Response::Success {
        data: "Operation completed".to_string(),
    };
    let success_debug: String = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", success))
    });
    if !success_debug.contains("Operation completed") {
        ::core::panicking::panic(
            "assertion failed: success_debug.contains(\"Operation completed\")",
        )
    }
    let error: Response = Response::Error {
        message: "Something went wrong".to_string(),
        _internal_code: 500,
    };
    let error_debug: String = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", error))
    });
    if !error_debug.contains("Something went wrong") {
        ::core::panicking::panic(
            "assertion failed: error_debug.contains(\"Something went wrong\")",
        )
    }
    if !!error_debug.contains("500") {
        ::core::panicking::panic("assertion failed: !error_debug.contains(\"500\")")
    }
    let person = Person::new("Alice".to_string(), 30);
    match (&person.name, &"Alice") {
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
    let user = User::new("alice".to_string(), "alice".to_string());
    match (&user.email, &None) {
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
    let product = Product::new(1, "Laptop".to_string(), 999.99);
    match (&*product.get_id(), &1) {
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
    match (&product.get_name(), &"Laptop") {
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
    let tuple_point = TuplePoint::new(10.5, 30.5);
    match (&tuple_point.0, &10.5) {
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
    match (&tuple_point.1, &0.0) {
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
    match (&tuple_point.2, &30.5) {
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
    let public_person = PublicPerson::new("Alice".to_string(), 25);
    match (&public_person._name, &"Alice") {
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
    match (&public_person._age, &25) {
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
    let crate_person = CratePerson::new("Bob".to_string(), 35);
    match (&crate_person._name, &"Bob") {
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
    match (&crate_person._age, &35) {
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
    let private_person = PrivatePerson::new("Charlie".to_string(), 45);
    match (&private_person._name, &"Charlie") {
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
    match (&private_person._age, &45) {
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
    let mut trait_test = TraitTestStruct {
        name: "test".to_string(),
        value: 42,
        data: <[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3])),
        items: <[_]>::into_vec(
            ::alloc::boxed::box_new(["item1".to_string(), "item2".to_string()]),
        ),
    };
    trait_test.set_name("new name");
    trait_test.set_value(100);
    trait_test.set_data([4, 5, 6, 7]);
    let new_items = <[_]>::into_vec(
        ::alloc::boxed::box_new(["new1".to_string(), "new2".to_string()]),
    );
    trait_test.set_items(new_items);
    match (&*trait_test.get_name(), &"new name") {
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
    match (&trait_test.get_value(), &100) {
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
    match (
        &*trait_test.get_data(),
        &<[_]>::into_vec(::alloc::boxed::box_new([4, 5, 6, 7])),
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
    match (
        &*trait_test.get_items(),
        &<[_]>::into_vec(
            ::alloc::boxed::box_new(["new1".to_string(), "new2".to_string()]),
        ),
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
    let nested = NestedStruct {
        name: "inner".to_string(),
        _value: 42,
    };
    let mut complex = ComplexNestedStruct {
        nested: nested.clone(),
        nested_list: <[_]>::into_vec(::alloc::boxed::box_new([nested])),
        metadata: std::collections::HashMap::new(),
    };
    complex
        .set_metadata({
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), "value".to_string());
            map
        });
    match (&complex.get_nested().get_name(), &"inner") {
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
    match (&complex.get_nested_list().len(), &1) {
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
    match (&complex.get_metadata().get("key").unwrap(), &"value") {
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
    let simple = ComplexEnum::Simple;
    let tuple = ComplexEnum::Tuple("test".to_string(), 123);
    let struct_variant = ComplexEnum::Struct {
        field1: "visible".to_string(),
        _secret: "hidden".to_string(),
        value: PI,
    };
    let simple_debug = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", simple))
    });
    let tuple_debug = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", tuple))
    });
    let struct_debug = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0:?}", struct_variant))
    });
    if !simple_debug.contains("Simple") {
        ::core::panicking::panic("assertion failed: simple_debug.contains(\"Simple\")")
    }
    if !tuple_debug.contains("test") {
        ::core::panicking::panic("assertion failed: tuple_debug.contains(\"test\")")
    }
    if !tuple_debug.contains("123") {
        ::core::panicking::panic("assertion failed: tuple_debug.contains(\"123\")")
    }
    if !struct_debug.contains("visible") {
        ::core::panicking::panic("assertion failed: struct_debug.contains(\"visible\")")
    }
    if !!struct_debug.contains("hidden") {
        ::core::panicking::panic("assertion failed: !struct_debug.contains(\"hidden\")")
    }
    if !struct_debug.contains("3.14") {
        ::core::panicking::panic("assertion failed: struct_debug.contains(\"3.14\")")
    }
    let generic_i32 = GenericStruct::<i32> {
        data: 0,
        value: 100,
    };
    let generic_string = GenericStruct::<String>::new(200);
    match (&generic_i32.value, &100) {
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
    match (&generic_i32.data, &0) {
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
    match (&generic_string.value, &200) {
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
    match (&generic_string.data, &"") {
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
    let name = "rust";
    let description = "language";
    let lifetimes_test = LifetimesTest { name, description };
    match (&*lifetimes_test.get_name(), &"rust") {
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
    match (&*lifetimes_test.get_description(), &"language") {
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
    let edge_case = EdgeCaseTest {
        empty_string: String::new(),
        empty_vec: Vec::new(),
        zero_value: 0,
        bool_false: false,
        option_none: None,
    };
    match (&edge_case.get_empty_string(), &"") {
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
    if !edge_case.get_empty_vec().is_empty() {
        ::core::panicking::panic(
            "assertion failed: edge_case.get_empty_vec().is_empty()",
        )
    }
    match (&edge_case.get_zero_value(), &0) {
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
    if !!edge_case.get_bool_false() {
        ::core::panicking::panic("assertion failed: !edge_case.get_bool_false()")
    }
    if !edge_case.try_get_option_none().is_none() {
        ::core::panicking::panic(
            "assertion failed: edge_case.try_get_option_none().is_none()",
        )
    }
    let unit_get = UnitGetSet { flag: true };
    let flag_ref = unit_get.get_flag();
    if !*flag_ref {
        ::core::panicking::panic("assertion failed: *flag_ref")
    }
    let constructed = AllSkipped::new();
    match (&constructed.skipped1, &"") {
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
    match (&constructed.skipped2, &0) {
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
    let copy_test = CopyTest {
        value: 42,
        flag: true,
        count: 1000,
    };
    let copied_value: i32 = copy_test.get_value();
    let copied_flag: bool = copy_test.get_flag();
    let copied_count: u64 = copy_test.get_count();
    match (&copied_value, &42) {
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
    if !copied_flag {
        ::core::panicking::panic("assertion failed: copied_flag")
    }
    match (&copied_count, &1000) {
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
