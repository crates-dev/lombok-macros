use proc_macro::TokenStream;
pub(crate) mod cfg;
pub(crate) mod func;
pub(crate) mod generate;
pub(crate) mod parse;
pub(crate) mod visibility;
use generate::func::inner_lombok_data;

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
#[proc_macro_derive(Lombok, attributes(set, get))]
pub fn lombok_data(input: TokenStream) -> TokenStream {
    inner_lombok_data(input)
}
