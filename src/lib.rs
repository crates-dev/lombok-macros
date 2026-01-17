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
    Data, DeriveInput, Field, Fields, GenericArgument,
    GenericParam::{self},
    Generics, Ident, Index, Lifetime, PathArguments,
    Type::{self},
    TypeParam, Variant, WhereClause, parse_macro_input,
};

/// A procedural macro that automatically generates getter methods for struct and enum fields.
///
/// This macro derives getter methods with configurable visibility and return type behavior.
/// The generated getters can return either references to field values or cloned copies,
/// with support for Option and Result types.
///
/// # Supported Attributes
/// - `#[get(pub)]`: Generates a public getter with reference return type
/// - `#[get(pub, reference)]`: Generates a public getter that returns a reference (`&T`)
/// - `#[get(pub, clone)]`: Generates a public getter that returns a cloned value (`T`)
/// - `#[get(pub, copy)]`: Generates a public getter that returns a copy of the field value (`self.field`) for Copy types
/// - `#[get(pub, deref)]`: Generates a public getter that returns a dereferenced value (`*field`) with enhanced match control for Option/Result types
/// - `#[get(pub(crate))]`: Generates a crate-visible getter
/// - `#[get(private)]`: Generates a private getter
///
/// # Return Type Behavior
/// - `reference`: Returns `&T` - a reference to the field value
/// - `clone`: Returns `T` - a cloned copy of the field value  
/// - `copy`: Returns `T` - a copy of the field value (`self.field`) for types implementing Copy trait
/// - `deref`: Returns dereferenced values with enhanced match control:
///   - `Option<T>` → `T` with detailed None panic messages
///   - `Result<T, E>` → `T` with detailed Err panic messages
///   - `Box<T>` → `T` by dereferencing the box
///   - `Rc<T>` → `T` by cloning the inner value
///   - `Arc<T>` → `T` by cloning the inner value
///   - Other types → `T` by dereferencing
/// - Default behavior: Returns `&T` for non-Option/Result types, `T` for Option/Result types
///
/// # Default Behavior Details
/// - **Non-Option/Result types**: Returns `&T` (reference to field)
/// - **Option/Result types**: Returns `T` (cloned value) to avoid exposing internal references
/// - This ensures safe access patterns while maintaining performance for common use cases
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct BasicStruct {
///     #[get(pub)]
///     name: String,
///     #[get(pub, reference)]
///     description: String,
///     #[get(pub, clone)]
///     data: Vec<i32>,
///     #[get(pub, copy)]
///     count: i32,
/// }
/// let basic = BasicStruct {
///     name: "test".to_string(),
///     description: "description".to_string(),
///     data: vec![1, 2, 3],
///     count: 42,
/// };
/// let name_ref: &String = basic.get_name();
/// let description_ref: &String = basic.get_description();
/// let data_clone: Vec<i32> = basic.get_data();
/// let count_copy: i32 = basic.get_count();
/// assert_eq!(*name_ref, "test");
/// assert_eq!(*description_ref, "description");
/// assert_eq!(data_clone, vec![1, 2, 3]);
/// assert_eq!(count_copy, 42);
/// ```
///
/// ## Option and Result Types
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct OptionalStruct {
///     #[get(pub)]
///     optional: Option<String>,
///     #[get(pub, reference)]
///     optional_ref: Option<String>,
///     #[get(pub)]
///     result: Result<String, String>,
/// }
/// let opt_struct = OptionalStruct {
///     optional: Some("value".to_string()),
///     optional_ref: Some("ref_value".to_string()),
///     result: Ok("success".to_string()),
/// };
/// let optional_value: String = opt_struct.get_optional();
/// let optional_reference: &Option<String> = opt_struct.get_optional_ref();
/// let result_value: String = opt_struct.get_result();
/// assert_eq!(optional_value, "value");
/// assert_eq!(*optional_reference, Some("ref_value".to_string()));
/// assert_eq!(result_value, "success");
/// ```
///
/// ## Tuple Structs
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct TupleStruct(
///     #[get(pub)] String,
///     #[get(pub, clone)] Vec<i32>,
/// );
/// let tuple = TupleStruct("hello".to_string(), vec![1, 2, 3]);
/// let field0: &String = tuple.get_0();
/// let field1: Vec<i32> = tuple.get_1();
/// assert_eq!(*field0, "hello");
/// assert_eq!(field1, vec![1, 2, 3]);
/// ```
///
/// ## Deref Return Type with Enhanced Match Control
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct DerefStruct {
///     #[get(deref)]
///     optional: Option<bool>,
///     #[get(pub, deref)]
///     result: Result<String, &'static str>,
///     #[get(pub, deref)]
///     boxed_value: Box<i32>,
///     #[get(pub, deref)]
///     rc_value: std::rc::Rc<String>,
///     #[get(pub, deref)]
///     arc_value: std::sync::Arc<Vec<u8>>,
/// }
/// let deref_struct = DerefStruct {
///     optional: Some(true),
///     result: Ok("success".to_string()),
///     boxed_value: Box::new(100),
///     rc_value: std::rc::Rc::new("test".to_string()),
///     arc_value: std::sync::Arc::new(vec![1, 2, 3]),
/// };
/// let optional_value: bool = deref_struct.get_optional();
/// let result_value: String = deref_struct.get_result();
/// let boxed_value: i32 = deref_struct.get_boxed_value();
/// let rc_value: String = deref_struct.get_rc_value();
/// let arc_value: Vec<u8> = deref_struct.get_arc_value();
/// assert_eq!(optional_value, true);
/// assert_eq!(result_value, "success");
/// assert_eq!(boxed_value, 100);
/// assert_eq!(rc_value, "test");
/// assert_eq!(arc_value, vec![1, 2, 3]);
/// ```
///
/// ## Generics and Lifetimes
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Getter, Clone)]
/// struct GenericStruct<'a, T: Clone> {
///     #[get(pub)]
///     value: &'a T,
///     #[get(pub, clone)]
///     owned: T,
/// }
/// let data = 42;
/// let generic = GenericStruct {
///     value: &data,
///     owned: 42,
/// };
/// let value_ref: &i32 = generic.get_value();
/// let owned_clone: i32 = generic.get_owned();
/// assert_eq!(*value_ref, 42);
/// assert_eq!(owned_clone, 42);
/// ```
#[proc_macro_derive(Getter, attributes(get))]
pub fn getter(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, true, false, false)
}

/// A procedural macro that automatically generates mutable getter methods for struct and enum fields.
///
/// This macro derives mutable getter methods that provide mutable references to field values,
/// allowing modification of the struct's fields while maintaining proper borrowing semantics.
///
/// # Supported Attributes
/// - `#[get_mut(pub)]`: Generates a public mutable getter
/// - `#[get_mut(pub(crate))]`: Generates a crate-visible mutable getter
/// - `#[get_mut(pub(super))]`: Generates a mutable getter visible to parent module
/// - `#[get_mut(private)]`: Generates a private mutable getter
///
/// # Example
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(GetterMut, Clone)]
/// struct StructWithLifetimes<'a, 'b, T: Clone> {
///     #[get_mut(pub(crate))]
///     list: Vec<String>,
///     #[get_mut(pub(crate))]
///     optional_lifetime_a: Option<&'a T>,
///     optional_lifetime_b: Option<&'b str>,
/// }
/// let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// let mut struct_with_lifetimes: StructWithLifetimes<usize> = StructWithLifetimes {
///     list: list.clone(),
///     optional_lifetime_a: None,
///     optional_lifetime_b: None,
/// };
/// let mut list_reference: &mut Vec<String> = struct_with_lifetimes.get_mut_list();
/// list_reference.push("new_item".to_string());
/// assert_eq!(*list_reference, vec!["hello".to_string(), "world".to_string(), "new_item".to_string()]);
/// ```
#[proc_macro_derive(GetterMut, attributes(get_mut))]
pub fn getter_mut(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, false, true, false)
}

/// A procedural macro that automatically generates setter methods for struct and enum fields.
///
/// This macro derives setter methods that allow modification of struct fields with
/// configurable visibility and parameter type conversion options.
///
/// # Supported Attributes
/// - `#[set(pub)]`: Generates a public setter
/// - `#[set(pub(crate))]`: Generates a crate-visible setter
/// - `#[set(pub(super))]`: Generates a setter visible to parent module
/// - `#[set(private)]`: Generates a private setter
/// - `#[set(pub, Into)]`: Generates a setter that accepts any type implementing Into<T>
/// - `#[set(pub, AsRef)]`: Generates a setter that accepts types implementing AsRef<T>
///
/// # Parameter Conversion
/// - `Into`: Enables accepting any type that implements `Into<T>` for the field type
/// - `AsRef`: Enables accepting types that implement `AsRef<T>` for the field type
/// - Default: Accepts the exact field type
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Setter, Debug, Clone)]
/// struct BasicStruct {
///     #[set(pub)]
///     name: String,
///     #[set(pub(crate))]
///     value: i32,
///     #[set(private)]
///     secret: String,
/// }
/// let mut basic = BasicStruct {
///     name: "initial".to_string(),
///     value: 0,
///     secret: "hidden".to_string(),
/// };
/// basic.set_name("updated".to_string());
/// basic.set_value(42);
/// assert_eq!(basic.name, "updated");
/// assert_eq!(basic.value, 42);
/// ```
///
/// ## Trait-Based Parameter Conversion
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Setter, Debug, Clone)]
/// struct ConversionStruct {
///     #[set(pub, Into)]
///     name: String,
///     #[set(pub, AsRef)]
///     description: String,
/// }
/// let mut conversion_struct = ConversionStruct {
///     name: "initial".to_string(),
///     description: "desc".to_string(),
/// };
///
/// conversion_struct.set_name("from_str");
/// conversion_struct.set_description("from_str_ref");
///
/// assert_eq!(conversion_struct.name, "from_str");
/// assert_eq!(conversion_struct.description, "from_str_ref");
/// ```
///
/// ## Tuple Structs
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Setter, Debug, Clone)]
/// struct TupleStruct(
///     #[set(pub)] String,
///     #[set(pub)] i32,
/// );
/// let mut tuple = TupleStruct("hello".to_string(), 1);
/// tuple.set_0("world".to_string());
/// tuple.set_1(100);
/// assert_eq!(tuple.0, "world");
/// assert_eq!(tuple.1, 100);
/// ```
#[proc_macro_derive(Setter, attributes(set))]
pub fn setter(input: TokenStream) -> TokenStream {
    inner_lombok_data(input, false, false, true)
}

/// A procedural macro that combines getter, mutable getter, and setter functionality in a single derive.
///
/// This macro derives all three types of accessor methods (getters, mutable getters, and setters)
/// for struct and enum fields, providing comprehensive data manipulation capabilities with
/// configurable visibility and behavior options.
///
/// # Supported Attributes
/// - `#[get(...)]`: Controls getter generation (supports `reference`, `clone` options)
/// - `#[get_mut(...)]`: Controls mutable getter generation
/// - `#[set(...)]`: Controls setter generation (supports `Into`, `AsRef` options)
///
/// # Visibility Control
/// Each attribute supports the same visibility options:
/// - `pub`: Public access
/// - `pub(crate)`: Crate-level access
/// - `pub(super)`: Parent module access
/// - `private`: Private access
///
/// # Examples
///
/// ## Basic Combination
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Data, Debug, Clone)]
/// struct User {
///     #[get(pub)]
///     #[set(pub)]
///     name: String,
///     #[get(pub, clone)]
///     #[set(pub, Into)]
///     email: String,
///     #[get_mut(pub)]
///     age: u32,
/// }
///
/// let mut user = User {
///     name: "Alice".to_string(),
///     email: "alice@example.com".to_string(),
///     age: 30,
/// };
///
/// let name_reference: &String = user.get_name();
/// let email_clone: String = user.get_email();
/// assert_eq!(*name_reference, "Alice");
/// assert_eq!(email_clone, "alice@example.com");
///
/// user.set_name("Bob".to_string());
/// user.set_email("bob@example.com");
///
/// let updated_email: String = user.get_email();
/// assert_eq!(updated_email, "bob@example.com");
///
/// let age_mutable_reference: &mut u32 = user.get_mut_age();
/// *age_mutable_reference = 31;
///
/// assert_eq!(*age_mutable_reference, 31);
/// ```
///
/// ## Multiple Field Types
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Data, Debug, Clone)]
/// struct ComplexStruct {
///     #[get(pub)]
///     id: i32,
///     #[get(pub)]
///     #[set(pub)]
///     optional: Option<String>,
///     #[get(pub, reference)]
///     result: Result<i32, String>,
///     #[get(pub(crate))]
///     #[set(private)]
///     internal_data: Vec<u8>,
/// }
///
/// let mut complex = ComplexStruct {
///     id: 1,
///     optional: Some("value".to_string()),
///     result: Ok(42),
///     internal_data: vec![1, 2, 3],
/// };
///
/// let id_reference: &i32 = complex.get_id();
/// let optional_clone: String = complex.get_optional();
/// let result_reference: &Result<i32, String> = complex.get_result();
///
/// assert_eq!(*id_reference, 1);
/// assert_eq!(optional_clone, "value");
/// assert_eq!(*result_reference, Ok(42));
/// ```
///
/// ## Tuple Struct with Combined Accessors
///
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(Data, Debug, Clone)]
/// struct Point(
///     #[get(pub)] f64,
///     #[get(pub, clone)]
///     #[set(pub)] f64,
/// );
///
/// let mut point = Point(1.0, 2.0);
/// let x_coordinate: &f64 = point.get_0();
/// let y_coordinate: f64 = point.get_1();
///
/// assert_eq!(*x_coordinate, 1.0);
/// assert_eq!(y_coordinate, 2.0);
///
/// point.set_1(3.0);
/// let updated_y_coordinate: f64 = point.get_1();
/// assert_eq!(updated_y_coordinate, 3.0);
/// ```
#[proc_macro_derive(Data, attributes(get, get_mut, set))]
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
/// assert_eq!(user.created_at, "");
/// ```
///
/// ## With Custom Visibility
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// #[new(pub(crate))]
/// struct InternalStruct {
///     value: i32,
/// }
///
/// let internal = InternalStruct::new(42);
/// assert_eq!(internal.value, 42);
/// ```
///
/// ## Tuple Structs
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// struct Point(
///     f64,
///     f64,
/// );
///
/// let origin = Point::new(0.0, 0.0);
/// assert_eq!(origin.0, 0.0);
/// assert_eq!(origin.1, 0.0);
/// ```
///
/// ## Generic Types
/// ```rust
/// use lombok_macros::*;
///
/// #[derive(New)]
/// struct Container<T: Default + Clone> {
///     data: T,
///     #[new(skip)]
///     count: usize,
/// }
///
/// let container = Container::new("data".to_string());
/// assert_eq!(container.data, "data");
/// assert_eq!(container.count, 0);
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
