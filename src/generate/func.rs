use super::constant::{
    FIELD_SHOULD_HAVE_A_NAME, GET_METHOD_PREFIX, GET_MUT_METHOD_PREFIX, SET_METHOD_PREFIX,
    UNSUPPORTED_LOMBOK_DERIVE,
};
use crate::{cfg::r#type::Cfg, parse::func::analyze_attributes};
use proc_macro::TokenStream as OldTokenStream;
use proc_macro2::TokenStream as NewTokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{
    parse_macro_input, Data, DeriveInput, Field,
    GenericParam::{self},
    Ident, Lifetime,
    Type::{self},
    TypeParam,
};

/// Generates getter and setter functions for a given struct field.
///
/// # Parameters
/// - `field`: A reference to the `Field` structure representing the field for which the getter and setter will be generated.
///
/// # Returns
/// - A `NewTokenStream` containing the generated getter and setter functions.
pub fn generate_getter_setter(field: &Field) -> NewTokenStream {
    let attr_name: String = field
        .ident
        .as_ref()
        .expect(FIELD_SHOULD_HAVE_A_NAME)
        .to_string();
    let attr_name_ident: &Ident = field.ident.as_ref().unwrap();
    let attr_ty: &Type = &field.ty;
    let get_name: Ident = format_ident!("{}{}", GET_METHOD_PREFIX, attr_name);
    let get_mut_name: Ident = format_ident!("{}{}", GET_MUT_METHOD_PREFIX, attr_name);
    let set_name: Ident = format_ident!("{}{}", SET_METHOD_PREFIX, attr_name);
    let mut generated: NewTokenStream = quote! {};
    let mut cfg_map: HashMap<String, Vec<Cfg>> = HashMap::new();
    for attr in &field.attrs {
        let cfg: Cfg = analyze_attributes(attr.to_token_stream());
        let name: String = field.ident.to_token_stream().to_string();
        cfg_map.entry(name).or_insert_with(Vec::new).push(cfg);
    }
    let get_quote = |vis: NewTokenStream| {
        quote! {
            #[inline]
            #vis fn #get_name(&self) -> &#attr_ty {
                &self.#attr_name_ident
            }
            #[inline]
            #vis fn #get_mut_name(&mut self) -> &mut #attr_ty {
                &self.#attr_name_ident
            }
        }
    };
    let set_quote = |vis: NewTokenStream| {
        quote! {
            #[inline]
            #vis fn #set_name(&mut self, val: #attr_ty) -> &mut Self {
                self.#attr_name_ident = val;
                self
            }
        }
    };
    let mut has_add_get: bool = false;
    let mut has_add_set: bool = false;
    for (_key, cfg_list) in &cfg_map {
        for cfg in cfg_list {
            if has_add_get && has_add_set {
                break;
            }
            if cfg.skip && cfg.func_type.is_unknown() {
                continue;
            }
            let vis: NewTokenStream = cfg.visibility.to_token_stream();
            if cfg.func_type.is_get() {
                if !cfg.skip && !has_add_get {
                    generated.extend(get_quote(vis.clone()));
                }
                has_add_get = true;
            }
            if cfg.func_type.is_set() {
                if !cfg.skip && !has_add_set {
                    generated.extend(set_quote(vis.clone()));
                }
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_set {
        let cfg: Cfg = Cfg::default();
        let vis: NewTokenStream = cfg.visibility.to_token_stream();
        if !has_add_get {
            generated.extend(get_quote(vis.clone()));
        }
        if !has_add_set {
            generated.extend(set_quote(vis.clone()));
        }
    }
    generated
}

/// Processes the input token stream to generate `Lombok`-style boilerplate code.
///
/// # Parameters
/// - `input`: An `OldTokenStream` representing the input tokens to be processed.
///
/// # Returns
/// - An `OldTokenStream` containing the transformed tokens with `Lombok`-style data code.
pub fn inner_lombok_data(input: OldTokenStream) -> OldTokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let type_bounds: Vec<TypeParam> = input
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.clone())
            } else {
                None
            }
        })
        .collect();
    let types: Vec<Ident> = input
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        })
        .collect();
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
        _ => panic!("{}", UNSUPPORTED_LOMBOK_DERIVE),
    };
    let expanded: NewTokenStream = if lifetimes.is_empty() {
        if type_bounds.is_empty() {
            quote! {
                impl #name {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: NewTokenStream = quote! { #(#type_bounds),* };
            let type_generics: NewTokenStream = quote! { #(#types),* };
            quote! {
                impl<#type_bounds_generics> #name<#type_generics> {
                    #(#methods)*
                }
            }
        }
    } else {
        let lifetimes_generics: NewTokenStream = quote! { #(#lifetimes),* };
        if type_bounds.is_empty() {
            quote! {
                impl<#lifetimes_generics> #name<#lifetimes_generics> {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: NewTokenStream = quote! { #(#type_bounds),* };
            let type_generics: NewTokenStream = quote! { #(#types),* };
            quote! {
                impl<#lifetimes_generics, #type_bounds_generics> #name<#lifetimes_generics, #type_generics> {
                    #(#methods)*
                }
            }
        }
    };
    expanded.into()
}
