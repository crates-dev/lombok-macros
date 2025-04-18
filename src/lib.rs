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

/// This is an example of how to use the `Lombok` procedural macro with `get` and `set` attributes.
///
/// The `Lombok` procedural macro is used to automatically generate getters and setters for struct fields.
/// The `get` attribute controls the visibility of the getter function, and the `set` attribute controls
/// the visibility of the setter function.
///
/// Example:
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Lombok, Debug, Clone)]
/// struct LombokTest<'a, 'b, T: Clone> {
///     #[get(pub(crate))]
///     #[set(pub(crate))]
///     list: Vec<String>,
///     #[get(pub(crate))]
///     opt_str_lifetime_a: Option<&'a T>,
///     #[set(private)]
///     opt_str_lifetime_b: Option<&'b str>,
/// }
///
/// fn main() {
///     let mut data: LombokTest<usize> = LombokTest {
///         list: Vec::new(),
///         opt_str_lifetime_a: None,
///         opt_str_lifetime_b: None,
///     };
///     let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
///     data.set_list(list.clone());
///     match data.get_list() {
///         left_val => {
///             assert_eq!(*left_val, list);
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Lombok, attributes(set, get, get_mut))]
pub fn lombok_data(input: TokenStream) -> TokenStream {
    inner_lombok_data(input)
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
