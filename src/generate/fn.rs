use crate::*;

fn get_clean_attr_name(attr_str: &str) -> String {
    let clean_attr: String = if let Some(stripped) = attr_str.strip_prefix("r#") {
        stripped.to_owned()
    } else {
        attr_str.to_owned()
    };
    clean_attr
}

/// Generates getter and setter functions for a given struct field.
///
/// # Parameters
/// - `field`: A reference to the `Field` structure representing the field for which the getter and setter will be generated.
/// - `field_index`: Optional index for tuple struct fields.
/// - `need_getter`: A boolean indicating whether a getter function should be generated.
/// - `need_setter`: A boolean indicating whether a setter function should be generated.
///
/// # Returns
/// - A `NewTokenStream` containing the generated getter and setter functions.
pub fn generate_getter_setter(
    field: &Field,
    field_index: Option<usize>,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> NewTokenStream {
    if let Some(index) = field_index {
        generate_tuple_getter_setter(field, index, need_getter, need_getter_mut, need_setter)
    } else {
        generate_named_getter_setter(field, need_getter, need_getter_mut, need_setter)
    }
}

/// Generates getter and setter functions for named struct fields.
fn generate_named_getter_setter(
    field: &Field,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> NewTokenStream {
    let attr_name: String = field
        .ident
        .as_ref()
        .expect(FIELD_SHOULD_HAVE_A_NAME)
        .to_string();
    let attr_name_ident: &Ident = field.ident.as_ref().unwrap();
    let attr_ty: &Type = &field.ty;
    let clean_attr_name: String = get_clean_attr_name(&attr_name);
    let get_name: Ident = format_ident!("{}{}", GET_METHOD_PREFIX, clean_attr_name);
    let get_mut_name: Ident = format_ident!("{}{}", GET_MUT_METHOD_PREFIX, clean_attr_name);
    let set_name: Ident = format_ident!("{}{}", SET_METHOD_PREFIX, clean_attr_name);
    let mut generated: NewTokenStream = quote! {};
    let mut cfg_map: HashMap<String, Vec<Cfg>> = HashMap::new();
    for attr in &field.attrs {
        let cfg: Cfg = analyze_attributes(attr.to_token_stream());
        let name: String = field.ident.to_token_stream().to_string();
        cfg_map.entry(name).or_insert_with(Vec::new).push(cfg);
    }
    let get_quote = |vis: NewTokenStream| {
        if need_getter {
            quote! {
                #vis fn #get_name(&self) -> &#attr_ty {
                    &self.#attr_name_ident
                }
            }
        } else {
            quote! {}
        }
    };
    let get_mut_quote = |vis: NewTokenStream| {
        if need_getter_mut {
            quote! {
                #vis fn #get_mut_name(&mut self) -> &mut #attr_ty {
                    &mut self.#attr_name_ident
                }
            }
        } else {
            quote! {}
        }
    };
    let set_quote = |vis: NewTokenStream| {
        if need_setter {
            quote! {
                #vis fn #set_name(&mut self, val: #attr_ty) -> &mut Self {
                    self.#attr_name_ident = val;
                    self
                }
            }
        } else {
            quote! {}
        }
    };
    let mut has_add_get: bool = false;
    let mut has_add_get_mut: bool = false;
    let mut has_add_set: bool = false;
    for (_key, cfg_list) in &cfg_map {
        for cfg in cfg_list {
            if has_add_get && has_add_set && has_add_get_mut {
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
            if cfg.func_type.is_get_mut() {
                if !cfg.skip && !has_add_get_mut {
                    generated.extend(get_mut_quote(vis.clone()));
                }
                has_add_get_mut = true;
            }
            if cfg.func_type.is_set() {
                if !cfg.skip && !has_add_set {
                    generated.extend(set_quote(vis.clone()));
                }
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_set || !has_add_get_mut {
        let cfg: Cfg = Cfg::default();
        let vis: NewTokenStream = cfg.visibility.to_token_stream();
        if !has_add_get {
            generated.extend(get_quote(vis.clone()));
        }
        if !has_add_get_mut {
            generated.extend(get_mut_quote(vis.clone()));
        }
        if !has_add_set {
            generated.extend(set_quote(vis.clone()));
        }
    }
    generated
}

/// Generates getter and setter functions for tuple struct fields.
fn generate_tuple_getter_setter(
    field: &Field,
    index: usize,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> NewTokenStream {
    let attr_ty: &Type = &field.ty;
    let get_name: Ident = format_ident!("{}{}", GET_METHOD_PREFIX, index);
    let get_mut_name: Ident = format_ident!("{}{}", GET_MUT_METHOD_PREFIX, index);
    let set_name: Ident = format_ident!("{}{}", SET_METHOD_PREFIX, index);
    let field_index: syn::Index = syn::Index::from(index);
    let mut generated: NewTokenStream = quote! {};
    let mut cfg_map: HashMap<String, Vec<Cfg>> = HashMap::new();
    for attr in &field.attrs {
        let cfg: Cfg = analyze_attributes(attr.to_token_stream());
        let name: String = index.to_string();
        cfg_map.entry(name).or_insert_with(Vec::new).push(cfg);
    }
    let get_quote = |vis: NewTokenStream| {
        if need_getter {
            quote! {
                #vis fn #get_name(&self) -> &#attr_ty {
                    &self.#field_index
                }
            }
        } else {
            quote! {}
        }
    };
    let get_mut_quote = |vis: NewTokenStream| {
        if need_getter_mut {
            quote! {
                #vis fn #get_mut_name(&mut self) -> &mut #attr_ty {
                    &mut self.#field_index
                }
            }
        } else {
            quote! {}
        }
    };
    let set_quote = |vis: NewTokenStream| {
        if need_setter {
            quote! {
                #vis fn #set_name(&mut self, val: #attr_ty) -> &mut Self {
                    self.#field_index = val;
                    self
                }
            }
        } else {
            quote! {}
        }
    };
    let mut has_add_get: bool = false;
    let mut has_add_get_mut: bool = false;
    let mut has_add_set: bool = false;
    for (_key, cfg_list) in &cfg_map {
        for cfg in cfg_list {
            if has_add_get && has_add_set && has_add_get_mut {
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
            if cfg.func_type.is_get_mut() {
                if !cfg.skip && !has_add_get_mut {
                    generated.extend(get_mut_quote(vis.clone()));
                }
                has_add_get_mut = true;
            }
            if cfg.func_type.is_set() {
                if !cfg.skip && !has_add_set {
                    generated.extend(set_quote(vis.clone()));
                }
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_set || !has_add_get_mut {
        let cfg: Cfg = Cfg::default();
        let vis: NewTokenStream = cfg.visibility.to_token_stream();
        if !has_add_get {
            generated.extend(get_quote(vis.clone()));
        }
        if !has_add_get_mut {
            generated.extend(get_mut_quote(vis.clone()));
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
/// - `need_getter`: A boolean indicating whether getter functions should be generated.
/// - `need_getter_mut`: A boolean indicating whether mutable getter functions should be generated.
/// - `need_setter`: A boolean indicating whether setter functions should be generated.
///
/// # Returns
/// - An `OldTokenStream` containing the transformed tokens with `Lombok`-style data code.
pub fn inner_lombok_data(
    input: OldTokenStream,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> OldTokenStream {
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
    let where_clause: &Option<WhereClause> = &input.generics.where_clause;
    let methods: Vec<NewTokenStream> = match input.data {
        Data::Struct(ref s) => match &s.fields {
            syn::Fields::Named(_) => s
                .fields
                .iter()
                .map(|field| {
                    generate_getter_setter(field, None, need_getter, need_getter_mut, need_setter)
                })
                .collect::<Vec<_>>(),
            syn::Fields::Unnamed(_) => s
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    generate_getter_setter(
                        field,
                        Some(index),
                        need_getter,
                        need_getter_mut,
                        need_setter,
                    )
                })
                .collect::<Vec<_>>(),
            syn::Fields::Unit => Vec::new(),
        },
        _ => panic!("{}", UNSUPPORTED_LOMBOK_DERIVE),
    };
    let expanded: NewTokenStream = if lifetimes.is_empty() {
        if type_bounds.is_empty() {
            quote! {
                impl #name #where_clause {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: NewTokenStream = quote! { #(#type_bounds),* };
            let type_generics: NewTokenStream = quote! { #(#types),* };
            quote! {
                impl<#type_bounds_generics> #name<#type_generics> #where_clause {
                    #(#methods)*
                }
            }
        }
    } else {
        let lifetimes_generics: NewTokenStream = quote! { #(#lifetimes),* };
        if type_bounds.is_empty() {
            quote! {
                impl<#lifetimes_generics> #name<#lifetimes_generics> #where_clause {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: NewTokenStream = quote! { #(#type_bounds),* };
            let type_generics: NewTokenStream = quote! { #(#types),* };
            quote! {
                impl<#lifetimes_generics, #type_bounds_generics> #name<#lifetimes_generics, #type_generics> #where_clause {
                    #(#methods)*
                }
            }
        }
    };
    expanded.into()
}

/// Implements the `std::fmt::Display` trait for a given struct or enum.
///
/// This macro generates an implementation of the `Display` trait for a type,
/// allowing it to be formatted using `{}` in formatting macros. It takes
/// two parameters:
/// - `input`: The input `OldTokenStream` representing the Rust item (struct, enum, etc.)
/// - `is_format`: A boolean that controls the format used in `fmt`. If `true`, the format used
///   will be the debug format (`{:#?}`), which prints the value in a more human-readable form,
///   with indentation. If `false`, it will use the standard debug format (`{:?}`), which is
///   typically more compact.
///
/// # Parameters
/// - `input`: The input token stream that will be parsed as a `DeriveInput`.
/// - `is_format`: A boolean flag to determine the formatting style to use (`true` for `#?`,
///   `false` for `:?`).
///
/// # Returns
/// - `OldTokenStream`: The generated implementation of `std::fmt::Display` for the type.
pub(super) fn inner_display(input: OldTokenStream, is_format: bool) -> OldTokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let generics: &syn::Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded: NewTokenStream = if is_format {
        quote! {
            impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0:#?}", self))
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{:?}", self))
                }
            }
        }
    };
    OldTokenStream::from(expanded)
}

/// A wrapper function for `inner_display` that generates the `Display`
/// implementation with the standard debug format (`{:?}`).
///
/// This function is essentially a shorthand for calling `inner_display`
/// with the `is_format` parameter set to `false`, meaning it will use
/// the compact debug format.
///
/// # Parameters
/// - `input`: The input token stream that will be parsed as a `DeriveInput`.
///
/// # Returns
/// - `OldTokenStream`: The generated implementation of `std::fmt::Display`
///   for the type using the compact debug format.
pub fn inner_display_debug(input: OldTokenStream) -> OldTokenStream {
    inner_display(input, false)
}

/// A wrapper function for `inner_display` that generates the `Display`
/// implementation with the detailed debug format (`{:#?}`).
///
/// This function is essentially a shorthand for calling `inner_display`
/// with the `is_format` parameter set to `true`, meaning it will use
/// the more detailed debug format with indentation.
///
/// # Parameters
/// - `input`: The input token stream that will be parsed as a `DeriveInput`.
///
/// # Returns
/// - `OldTokenStream`: The generated implementation of `std::fmt::Display`
///   for the type using the detailed debug format.
pub fn inner_display_debug_format(input: OldTokenStream) -> OldTokenStream {
    inner_display(input, true)
}

/// Generates a custom Debug implementation that respects the `#[debug(skip)]` attribute.
///
/// This function creates a Debug implementation similar to the standard library's Debug,
/// but allows fields to be skipped from the debug output by using the `#[debug(skip)]` attribute.
///
/// # Parameters
/// - `input`: The input token stream representing the Rust item (struct, enum, etc.)
///   for which the Debug implementation will be generated.
///
/// # Returns
/// - `OldTokenStream`: The generated implementation of `std::fmt::Debug` for the type
///   that respects the `#[debug(skip)]` attribute.
pub fn inner_custom_debug(input: OldTokenStream) -> OldTokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let generics: &syn::Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    match &input.data {
        Data::Struct(data_struct) => {
            let fields: &syn::Fields = &data_struct.fields;
            match fields {
                syn::Fields::Named(_) => {
                    let debug_fields: Vec<NewTokenStream> = fields
                        .iter()
                        .filter_map(|field: &Field| {
                            let field_name: &Ident = field.ident.as_ref()?;
                            let mut should_skip: bool = false;
                            for attr in &field.attrs {
                                let cfg: Cfg = analyze_attributes(attr.to_token_stream());
                                if cfg.func_type.is_debug() && cfg.skip {
                                    should_skip = true;
                                    break;
                                }
                            }
                            if should_skip {
                                None
                            } else {
                                let field_name_str: String = field_name.to_string();
                                Some(quote! {
                                    .field(#field_name_str, &self.#field_name)
                                })
                            }
                        })
                        .collect();
                    let struct_name_str: String = name.to_string();
                    let expanded: NewTokenStream = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(#struct_name_str)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    };
                    OldTokenStream::from(expanded)
                }
                syn::Fields::Unnamed(_) => {
                    let debug_fields: Vec<NewTokenStream> = fields
                        .iter()
                        .enumerate()
                        .filter_map(|(i, field): (usize, &Field)| {
                            let mut should_skip: bool = false;
                            for attr in &field.attrs {
                                let cfg: Cfg = analyze_attributes(attr.to_token_stream());
                                if cfg.func_type.is_debug() && cfg.skip {
                                    should_skip = true;
                                    break;
                                }
                            }
                            if should_skip {
                                None
                            } else {
                                let field_index: syn::Index = syn::Index::from(i);
                                Some(quote! {
                                    .field(&self.#field_index)
                                })
                            }
                        })
                        .collect();
                    let struct_name_str: String = name.to_string();
                    let expanded: NewTokenStream = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_tuple(#struct_name_str)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    };
                    OldTokenStream::from(expanded)
                }
                syn::Fields::Unit => {
                    let struct_name_str: String = name.to_string();
                    let expanded: NewTokenStream = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(#struct_name_str).finish()
                            }
                        }
                    };
                    OldTokenStream::from(expanded)
                }
            }
        }
        Data::Enum(data_enum) => {
            let variants: Vec<NewTokenStream> = data_enum
                .variants
                .iter()
                .map(|variant: &syn::Variant| {
                    let variant_name: &Ident = &variant.ident;
                    let variant_name_str: String = variant_name.to_string();
                    match &variant.fields {
                        syn::Fields::Named(fields_named) => {
                            let field_patterns: Vec<NewTokenStream> = fields_named
                                .named
                                .iter()
                                .map(|field: &Field| {
                                    let field_name: &Ident = field.ident.as_ref().unwrap();
                                    quote! { #field_name }
                                })
                                .collect();
                            let debug_fields: Vec<NewTokenStream> = fields_named
                                .named
                                .iter()
                                .filter_map(|field: &Field| {
                                    let field_name: &Ident = field.ident.as_ref()?;
                                    let mut should_skip: bool = false;
                                    for attr in &field.attrs {
                                        let cfg: Cfg = analyze_attributes(attr.to_token_stream());
                                        if cfg.func_type.is_debug() && cfg.skip {
                                            should_skip = true;
                                            break;
                                        }
                                    }
                                    if should_skip {
                                        None
                                    } else {
                                        let field_name_str: String = field_name.to_string();
                                        Some(quote! {
                                            .field(#field_name_str, #field_name)
                                        })
                                    }
                                })
                                .collect();
                            quote! {
                                #name::#variant_name { #(#field_patterns),* } => {
                                    f.debug_struct(#variant_name_str)
                                        #(#debug_fields)*
                                        .finish()
                                }
                            }
                        }
                        syn::Fields::Unnamed(fields_unnamed) => {
                            let field_patterns: Vec<NewTokenStream> = fields_unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, _): (usize, &Field)| {
                                    let field_name: Ident = format_ident!("field_{}", i);
                                    quote! { #field_name }
                                })
                                .collect();
                            let debug_fields: Vec<NewTokenStream> = fields_unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .filter_map(|(i, field): (usize, &Field)| {
                                    let mut should_skip: bool = false;
                                    for attr in &field.attrs {
                                        let cfg: Cfg = analyze_attributes(attr.to_token_stream());
                                        if cfg.func_type.is_debug() && cfg.skip {
                                            should_skip = true;
                                            break;
                                        }
                                    }
                                    if should_skip {
                                        None
                                    } else {
                                        let field_name: Ident = format_ident!("field_{}", i);
                                        Some(quote! {
                                            .field(#field_name)
                                        })
                                    }
                                })
                                .collect();
                            quote! {
                                #name::#variant_name(#(#field_patterns),*) => {
                                    f.debug_tuple(#variant_name_str)
                                        #(#debug_fields)*
                                        .finish()
                                }
                            }
                        }
                        syn::Fields::Unit => {
                            quote! {
                                #name::#variant_name => {
                                    f.debug_struct(#variant_name_str).finish()
                                }
                            }
                        }
                    }
                })
                .collect();
            let expanded: NewTokenStream = quote! {
                impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variants)*
                        }
                    }
                }
            };
            OldTokenStream::from(expanded)
        }
        Data::Union(_) => {
            panic!("Debug derive is not supported for unions")
        }
    }
}
