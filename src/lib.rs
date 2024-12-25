use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::TokenStream as NewTokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Field,
    GenericParam::{self},
    Ident, Lifetime,
    Type::{self},
};

/// Generate getter and setter for a single field
/// This function generates a getter and setter for a given struct field.
///
/// # Parameters
/// - `field`: The struct field for which the getter and setter are generated.
///
/// # Returns
/// Returns a `TokenStream` containing the generated getter and setter methods.
fn generate_getter_setter(field: &Field) -> NewTokenStream {
    let attr_name: String = field
        .ident
        .as_ref()
        .expect("Field should have a name")
        .to_string();
    let attr_name_ident: &Ident = field.ident.as_ref().unwrap();
    let attr_ty: &Type = &field.ty;
    let get_name: Ident = format_ident!("get_{}", attr_name);
    let set_name: Ident = format_ident!("set_{}", attr_name);
    quote! {
        pub fn #get_name(&self) -> &#attr_ty {
            &self.#attr_name_ident
        }
        pub fn #set_name(&mut self, val: #attr_ty) -> &mut Self {
            self.#attr_name_ident = val;
            self
        }
    }
}

/// A procedural macro to automatically generate getter and setter methods for struct fields.
///
/// This macro can be applied to structs to automatically generate getter and setter methods for all
/// of its fields.
///
/// # Parameters
/// - `input`: The input token stream containing the struct to be processed.
///
/// # Returns
/// Returns the generated `TokenStream` containing the implementation of getter and setter methods
/// for the struct.
#[proc_macro_derive(Lombok)]
pub fn lombok_data(input: OldTokenStream) -> OldTokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let lifetimes: Vec<Lifetime> = input
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Lifetime(lifetime_param) = param {
                Some(lifetime_param.lifetime.clone())
            } else {
                None
            }
        })
        .collect();
    let methods: Vec<NewTokenStream> = match input.data {
        Data::Struct(ref s) => s
            .fields
            .iter()
            .map(generate_getter_setter)
            .collect::<Vec<_>>(),
        _ => panic!("#[derive(Lombok)] is only supported for structs."),
    };
    let expanded: NewTokenStream = if !lifetimes.is_empty() {
        let generics: NewTokenStream = quote! { #(#lifetimes),* };
        quote! {
            impl<#generics> #name<#generics> {
                #(#methods)*
            }
        }
    } else {
        quote! {
            impl #name {
                #(#methods)*
            }
        }
    };
    expanded.into()
}
