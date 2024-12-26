use proc_macro::TokenStream;
pub(crate) mod cfg;
pub(crate) mod func;
pub(crate) mod generate;
pub(crate) mod parse;
pub(crate) mod visibility;
use generate::func::inner_lombok_data;

#[proc_macro_derive(Lombok, attributes(set, get))]
pub fn lombok_data(input: TokenStream) -> TokenStream {
    inner_lombok_data(input)
}
