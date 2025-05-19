use proc_macro::TokenStream;
pub(crate) mod cfg;
pub(crate) mod func;
pub(crate) mod generate;
pub(crate) mod parse;
pub(crate) mod visibility;

pub(crate) use cfg::*;
pub(crate) use func::*;
pub(crate) use generate::*;
pub(crate) use parse::*;
pub(crate) use visibility::*;

pub(crate) use proc_macro::TokenStream as OldTokenStream;
pub(crate) use proc_macro2::{TokenStream as NewTokenStream, TokenTree as NewTokenTree};
pub(crate) use quote::{ToTokens, format_ident, quote};
pub(crate) use std::collections::HashMap;
pub(crate) use std::str::FromStr;
pub(crate) use syn::{
    Data, DeriveInput, Field,
    GenericParam::{self},
    Ident, Lifetime,
    Type::{self},
    TypeParam, parse_macro_input,
};

/// This is an example of how to use the `Lombok` procedural macro with `get` attributes.
///
/// The `Lombok` procedural macro is used to automatically generate getter methods for struct fields.
/// The `get` attribute controls the visibility of the generated getter method.
///
/// Example:
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct LombokTest2<'a, 'b, T: Clone> {
///     #[get(pub(crate))]
///     list: Vec<String>,
///     #[get(pub(crate))]
///     opt_str_lifetime_a: Option<&'a T>,
///     opt_str_lifetime_b: Option<&'b str>,
/// }
/// let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// let data2: LombokTest2<usize> = LombokTest2 {
///     list: list.clone(),
///     opt_str_lifetime_a: None,
///     opt_str_lifetime_b: None,
/// };
/// let get_list: Vec<String> = data2.get_list().clone();
/// assert_eq!(get_list, list);
/// ```
#[proc_macro_derive(Getter, attributes(get))]
pub fn getter(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, true, false, false)
}

/// This is an example of how to use the `Lombok` procedural macro with `get_mut` attributes.
///
/// The `Lombok` procedural macro is used to automatically generate mutable getters for struct fields.
/// The `get_mut` attribute controls the visibility of the mutable getter function.
///
/// Example:
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(GetterMut, Clone)]
/// struct LombokTest2<'a, 'b, T: Clone> {
///     #[get_mut(pub(crate))]
///     list: Vec<String>,
///     #[get_mut(pub(crate))]
///     opt_str_lifetime_a: Option<&'a T>,
///     opt_str_lifetime_b: Option<&'b str>,
/// }
/// let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// let mut data2: LombokTest2<usize> = LombokTest2 {
///     list: list.clone(),
///     opt_str_lifetime_a: None,
///     opt_str_lifetime_b: None,
/// };
/// let get_list: Vec<String> = data2.get_mut_list().clone();
/// assert_eq!(get_list, list);
/// ```
#[proc_macro_derive(GetterMut, attributes(get_mut))]
pub fn getter_mut(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, false, true, false)
}

/// This is an example of how to use the `Lombok` procedural macro with `set` attributes.
///
/// The `Lombok` procedural macro is used to automatically generate setters for struct fields.
/// The `set` attribute controls the visibility of the setter function.
///
/// Example:
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Setter, Debug, Clone)]
/// struct LombokTest<'a, 'b, T: Clone> {
///     #[set(pub(crate))]
///     list: Vec<String>,
///     opt_str_lifetime_a: Option<&'a T>,
///     #[set(private)]
///     opt_str_lifetime_b: Option<&'b str>,
/// }
/// let mut data: LombokTest<usize> = LombokTest {
///     list: Vec::new(),
///     opt_str_lifetime_a: None,
///     opt_str_lifetime_b: None,
/// };
/// let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// data.set_list(list.clone());
/// match data.list {
///     left_val => {
///         assert_eq!(*left_val, list);
///     }
/// }
/// ```
#[proc_macro_derive(Setter, attributes(set))]
pub fn setter(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, false, false, true)
}

/// This is an example of how to use the `Lombok` procedural macro with `get`, `get_mut`, and `set` attributes.
///
/// The `Lombok` procedural macro is used to automatically generate getters, mutable getters, and setters for struct fields.
/// The `get` and `get_mut` attributes control the visibility of the getter functions, while the `set` attribute controls
/// the visibility of the setter function.
///
/// Example:
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Data, Debug, Clone)]
/// struct LombokTest<'a, 'b, T: Clone> {
///     #[get(pub(crate))]
///     #[set(pub(crate))]
///     list: Vec<String>,
///     #[get(pub(crate))]
///     opt_str_lifetime_a: Option<&'a T>,
///     #[set(private)]
///     opt_str_lifetime_b: Option<&'b str>,
/// }
/// let mut data: LombokTest<usize> = LombokTest {
///     list: Vec::new(),
///     opt_str_lifetime_a: None,
///     opt_str_lifetime_b: None,
/// };
/// let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// data.set_list(list.clone());
/// match data.get_list() {
///     left_val => {
///         assert_eq!(*left_val, list);
///     }
/// }
/// ```
#[proc_macro_derive(Data, attributes(set, get, get_mut))]
pub fn data(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, true, true, true)
}

/// A procedural macro that implements the `std::fmt::Display` trait for a type,
/// using the standard debug format (`{:?}`) for formatting.
///
/// This macro derives the `Display` implementation for a type, allowing it to be formatted
/// using `{:?}` in formatting macros. It uses the `inner_display_debug` function to generate
/// the implementation with the standard debug format.
///
/// # Parameters
/// - `input`: The input token stream representing the Rust item (struct, enum, etc.)
///   for which the `Display` implementation will be generated.
///
/// # Returns
/// - `TokenStream`: The generated `std::fmt::Display` implementation for the type
///   using the standard debug format.
#[proc_macro_derive(DisplayDebug)]
pub fn display_debug(input: TokenStream) -> TokenStream {
    inner_display_debug(input)
}

/// A procedural macro that implements the `std::fmt::Display` trait for a type,
/// using the detailed debug format (`{:#?}`) for formatting.
///
/// This macro derives the `Display` implementation for a type, allowing it to be formatted
/// using `{:#?}` in formatting macros. It uses the `inner_display_debug_format` function
/// to generate the implementation with the detailed debug format.
///
/// # Parameters
/// - `input`: The input token stream representing the Rust item (struct, enum, etc.)
///   for which the `Display` implementation will be generated.
///
/// # Returns
/// - `TokenStream`: The generated `std::fmt::Display` implementation for the type
///   using the detailed debug format.
#[proc_macro_derive(DisplayDebugFormat)]
pub fn display_debug_format(input: TokenStream) -> TokenStream {
    inner_display_debug_format(input)
}
