//! lombok-macros
//!
//! A Rust procedural macro collection providing Lombok-like functionality.
//! Automatically generates getters/setters with field-level visibility control,
//! custom Debug implementations with field skipping, and Display trait implementations.
//! Supports structs, enums, generics and lifetimes.

pub(crate) mod config;
pub(crate) mod func;
pub(crate) mod generate;
pub(crate) mod parse;
pub(crate) mod visibility;

pub(crate) use config::*;
pub(crate) use func::*;
pub(crate) use generate::*;
pub(crate) use parse::*;
pub(crate) use visibility::*;

pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::{
    TokenStream as TokenStream2, TokenTree as TokenTree2, token_stream::IntoIter,
};
pub(crate) use quote::{ToTokens, format_ident, quote};
pub(crate) use std::{collections::HashMap, str::FromStr};
pub(crate) use syn::{
    Data, DeriveInput, Field,
    GenericParam::{self},
    Ident, Lifetime,
    Type::{self},
    TypeParam, WhereClause, parse_macro_input,
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
#[proc_macro_derive(Data, attributes(set, get, get_mut, new))]
pub fn data(input: TokenStream) -> TokenStream {
    let mut result: TokenStream2 = TokenStream2::new();
    let lombok_data: TokenStream = inner_lombok_data(input.clone(), true, true, true);
    result.extend(
        lombok_data
            .to_string()
            .parse::<TokenStream2>()
            .unwrap_or_default(),
    );
    result.into()
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

/// A procedural macro that implements the `std::fmt::Debug` trait for a type,
/// with support for the `#[debug(skip)]` attribute to skip specific fields.
///
/// This macro derives a custom Debug implementation that behaves like the standard
/// library's Debug derive, but allows individual fields to be excluded from the
/// debug output by annotating them with `#[debug(skip)]`.
///
/// # Supported Attributes
/// - `#[debug(skip)]`: Excludes the field from the debug output
///
/// # Examples
///
/// ## Struct Example
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(CustomDebug)]
/// struct User {
///     name: String,
///     #[debug(skip)]
///     password: String,
///     email: String,
/// }
///
/// let user = User {
///     name: "Alice".to_string(),
///     password: "secret123".to_string(),
///     email: "alice@example.com".to_string(),
/// };
/// let expected_debug = "User { name: \"Alice\", email: \"alice@example.com\" }";
/// assert_eq!(format!("{:?}", user), expected_debug);
/// ```
///
/// ## Enum Example
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(CustomDebug)]
/// enum Response {
///     Success { data: String },
///     Error {
///         message: String,
///         #[debug(skip)]
///         internal_code: u32,
///     },
/// }
///
/// let success = Response::Success { data: "Hello".to_string() };
/// let error = Response::Error { message: "Failed".to_string(), internal_code: 500 };
/// let expected_success = "Success { data: \"Hello\" }";
/// let expected_error = "Error { message: \"Failed\" }";
/// assert_eq!(format!("{:?}", success), expected_success);
/// assert_eq!(format!("{:?}", error), expected_error);
/// ```
///
/// # Parameters
/// - `input`: The input token stream representing the Rust item (struct, enum, etc.)
///   for which the Debug implementation will be generated.
///
/// # Returns
/// - `TokenStream`: The generated `std::fmt::Debug` implementation for the type
///   that respects the `#[debug(skip)]` attribute.
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn custom_debug(input: TokenStream) -> TokenStream {
    inner_custom_debug(input)
}

/// A procedural macro that generates a constructor function for structs.
///
/// This macro automatically generates a `new` function that takes all non-skipped fields
/// as parameters and returns a new instance of the struct. Fields marked with `#[new(skip)]`
/// will be initialized with their default values.
///
/// # Supported Attributes
/// - `#[new(skip)]`: Excludes the field from constructor parameters and uses default initialization
/// - `#[new(pub)]`: Generates a public constructor  
/// - `#[new(pub(crate))]`: Generates a crate-visible constructor  
/// - `#[new(pub(super))]`: Generates a constructor visible to parent module  
/// - `#[new(private)]`: Generates a private constructor
///
/// # Default Behavior
/// - The generated constructor is `pub` by default
/// - All fields are included in the constructor unless marked with `#[new(skip)]`
/// - Skipped fields are initialized using `Default::default()`
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let person = Person::new("Alice".to_string(), 30);
/// assert_eq!(person.name, "Alice");
/// assert_eq!(person.age, 30);
/// ```
///
/// ## With Skip Attribute
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// struct User {
///     username: String,
///     email: String,
///     #[new(skip)]
///     created_at: String,
/// }
///
/// let user = User::new("alice".to_string(), "alice@example.com".to_string());
/// assert_eq!(user.username, "alice");
/// assert_eq!(user.email, "alice@example.com");
/// assert_eq!(user.created_at, ""); // skipped field defaults to empty string
/// ```
///
/// ## With Visibility Control
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// #[new(pub(crate))]
/// struct InternalStruct {
///     value: i32,
/// }
/// ```
///
/// # Parameters
/// - `input`: The input token stream representing the struct for which to generate the constructor.
///
/// # Returns
/// - `TokenStream`: The generated constructor implementation.
#[proc_macro_derive(New, attributes(new))]
pub fn new(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let visibility: Visibility = parse_new_visibility(&derive_input);
    inner_new_constructor(&derive_input, visibility)
}
