use crate::*;

/// Cleans an attribute string by removing the "r#" prefix if present.
///
/// # Arguments
///
/// - `&str` - The attribute string to clean.
///
/// # Returns
///
/// - `String` - The cleaned attribute name.
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
/// # Arguments
///
/// - `&Field` - The field structure for which to generate getter/setter.
/// - `Option<usize>` - Optional index for tuple struct fields.
/// - `bool` - Whether to generate a getter function.
/// - `bool` - Whether to generate a mutable getter function.
/// - `bool` - Whether to generate a setter function.
///
/// # Returns
///
/// - `TokenStream2` - The generated getter and setter functions.
pub(crate) fn generate_getter_setter(
    field: &Field,
    field_index: Option<usize>,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> TokenStream2 {
    if let Some(index) = field_index {
        generate_tuple_getter_setter(field, index, need_getter, need_getter_mut, need_setter)
    } else {
        generate_named_getter_setter(field, need_getter, need_getter_mut, need_setter)
    }
}

/// Generates getter and setter functions for named struct fields.
///
/// # Arguments
///
/// - `&Field` - The field structure to generate for.
/// - `bool` - Whether to generate a getter function.
/// - `bool` - Whether to generate a mutable getter function.
/// - `bool` - Whether to generate a setter function.
///
/// # Returns
///
/// - `TokenStream2` - The generated getter and setter functions.
fn generate_named_getter_setter(
    field: &Field,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> TokenStream2 {
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
    let mut generated: TokenStream2 = quote! {};
    let mut config_map: HashMap<String, Vec<Config>> = HashMap::new();
    for attr in &field.attrs {
        let config: Config = analyze_attributes(attr.to_token_stream());
        let name: String = field.ident.to_token_stream().to_string();
        config_map.entry(name).or_default().push(config);
    }
    let get_quote = |vis: TokenStream2| {
        if need_getter {
            quote! {
                #[inline(always)]
                #vis fn #get_name(&self) -> &#attr_ty {
                    &self.#attr_name_ident
                }
            }
        } else {
            quote! {}
        }
    };
    let get_mut_quote = |vis: TokenStream2| {
        if need_getter_mut {
            quote! {
                #[inline(always)]
                #vis fn #get_mut_name(&mut self) -> &mut #attr_ty {
                    &mut self.#attr_name_ident
                }
            }
        } else {
            quote! {}
        }
    };
    let set_quote = |vis: TokenStream2| {
        if need_setter {
            quote! {
                #[inline(always)]
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
    for config_list in config_map.values() {
        for config in config_list {
            if has_add_get && has_add_set && has_add_get_mut {
                break;
            }
            if config.skip && config.func_type.is_unknown() {
                continue;
            }
            let vis: TokenStream2 = config.visibility.to_token_stream();
            if config.func_type.is_get() {
                if !config.skip && !has_add_get {
                    generated.extend(get_quote(vis.clone()));
                }
                has_add_get = true;
            }
            if config.func_type.is_get_mut() {
                if !config.skip && !has_add_get_mut {
                    generated.extend(get_mut_quote(vis.clone()));
                }
                has_add_get_mut = true;
            }
            if config.func_type.is_set() {
                if !config.skip && !has_add_set {
                    generated.extend(set_quote(vis.clone()));
                }
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_set || !has_add_get_mut {
        let config: Config = Config::default();
        let vis: TokenStream2 = config.visibility.to_token_stream();
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
///
/// # Arguments
///
/// - `&Field` - The field structure to generate for.
/// - `usize` - The index of the field in the tuple struct.
/// - `bool` - Whether to generate a getter function.
/// - `bool` - Whether to generate a mutable getter function.
/// - `bool` - Whether to generate a setter function.
///
/// # Returns
///
/// - `TokenStream2` - The generated getter and setter functions.
fn generate_tuple_getter_setter(
    field: &Field,
    index: usize,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> TokenStream2 {
    let attr_ty: &Type = &field.ty;
    let get_name: Ident = format_ident!("{}{}", GET_METHOD_PREFIX, index);
    let get_mut_name: Ident = format_ident!("{}{}", GET_MUT_METHOD_PREFIX, index);
    let set_name: Ident = format_ident!("{}{}", SET_METHOD_PREFIX, index);
    let field_index: syn::Index = syn::Index::from(index);
    let mut generated: TokenStream2 = quote! {};
    let mut config_map: HashMap<String, Vec<Config>> = HashMap::new();
    for attr in &field.attrs {
        let config: Config = analyze_attributes(attr.to_token_stream());
        let name: String = index.to_string();
        config_map.entry(name).or_default().push(config);
    }
    let get_quote = |vis: TokenStream2| {
        if need_getter {
            quote! {
                #[inline(always)]
                #vis fn #get_name(&self) -> &#attr_ty {
                    &self.#field_index
                }
            }
        } else {
            quote! {}
        }
    };
    let get_mut_quote = |vis: TokenStream2| {
        if need_getter_mut {
            quote! {
                #[inline(always)]
                #vis fn #get_mut_name(&mut self) -> &mut #attr_ty {
                    &mut self.#field_index
                }
            }
        } else {
            quote! {}
        }
    };
    let set_quote = |vis: TokenStream2| {
        if need_setter {
            quote! {
                #[inline(always)]
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
    for config_list in config_map.values() {
        for config in config_list {
            if has_add_get && has_add_set && has_add_get_mut {
                break;
            }
            if config.skip && config.func_type.is_unknown() {
                continue;
            }
            let vis: TokenStream2 = config.visibility.to_token_stream();
            if config.func_type.is_get() {
                if !config.skip && !has_add_get {
                    generated.extend(get_quote(vis.clone()));
                }
                has_add_get = true;
            }
            if config.func_type.is_get_mut() {
                if !config.skip && !has_add_get_mut {
                    generated.extend(get_mut_quote(vis.clone()));
                }
                has_add_get_mut = true;
            }
            if config.func_type.is_set() {
                if !config.skip && !has_add_set {
                    generated.extend(set_quote(vis.clone()));
                }
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_set || !has_add_get_mut {
        let config: Config = Config::default();
        let vis: TokenStream2 = config.visibility.to_token_stream();
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
/// # Arguments
///
/// - `TokenStream` - The input tokens to be processed.
/// - `bool` - Whether to generate getter functions.
/// - `bool` - Whether to generate mutable getter functions.
/// - `bool` - Whether to generate setter functions.
///
/// # Returns
///
/// - `TokenStream` - The transformed tokens with `Lombok`-style data code.
pub(crate) fn inner_lombok_data(
    input: TokenStream,
    need_getter: bool,
    need_getter_mut: bool,
    need_setter: bool,
) -> TokenStream {
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
    let methods: Vec<TokenStream2> = match input.data {
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
        _ => panic!("{}", UNSUPPORTED_DATA_DERIVE),
    };
    let expanded: TokenStream2 = if lifetimes.is_empty() {
        if type_bounds.is_empty() {
            quote! {
                impl #name #where_clause {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: TokenStream2 = quote! { #(#type_bounds),* };
            let type_generics: TokenStream2 = quote! { #(#types),* };
            quote! {
                impl<#type_bounds_generics> #name<#type_generics> #where_clause {
                    #(#methods)*
                }
            }
        }
    } else {
        let lifetimes_generics: TokenStream2 = quote! { #(#lifetimes),* };
        if type_bounds.is_empty() {
            quote! {
                impl<#lifetimes_generics> #name<#lifetimes_generics> #where_clause {
                    #(#methods)*
                }
            }
        } else {
            let type_bounds_generics: TokenStream2 = quote! { #(#type_bounds),* };
            let type_generics: TokenStream2 = quote! { #(#types),* };
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
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse.
/// - `bool` - Whether to use detailed debug format.
///
/// # Returns
///
/// - `TokenStream` - The generated `Display` implementation.
pub(super) fn inner_display(input: TokenStream, is_format: bool) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let generics: &syn::Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded: TokenStream2 = if is_format {
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
    TokenStream::from(expanded)
}

/// Generates `Display` implementation with standard debug format.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse.
///
/// # Returns
///
/// - `TokenStream` - The generated `Display` implementation.
pub(crate) fn inner_display_debug(input: TokenStream) -> TokenStream {
    inner_display(input, false)
}

/// Generates `Display` implementation with detailed debug format.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse.
///
/// # Returns
///
/// - `TokenStream` - The generated `Display` implementation.
pub(crate) fn inner_display_debug_format(input: TokenStream) -> TokenStream {
    inner_display(input, true)
}

/// Generates a custom Debug implementation respecting `#[debug(skip)]`.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to parse.
///
/// # Returns
///
/// - `TokenStream` - The generated `Debug` implementation.
pub(crate) fn inner_custom_debug(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: &Ident = &input.ident;
    let generics: &syn::Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    match &input.data {
        Data::Struct(data_struct) => {
            let fields: &syn::Fields = &data_struct.fields;
            match fields {
                syn::Fields::Named(_) => {
                    let debug_fields: Vec<TokenStream2> = fields
                        .iter()
                        .filter_map(|field: &Field| {
                            let field_name: &Ident = field.ident.as_ref()?;
                            let mut should_skip: bool = false;
                            for attr in &field.attrs {
                                let config: Config = analyze_attributes(attr.to_token_stream());
                                if config.func_type.is_debug() && config.skip {
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
                    let expanded: TokenStream2 = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(#struct_name_str)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    };
                    TokenStream::from(expanded)
                }
                syn::Fields::Unnamed(_) => {
                    let debug_fields: Vec<TokenStream2> = fields
                        .iter()
                        .enumerate()
                        .filter_map(|(i, field): (usize, &Field)| {
                            let mut should_skip: bool = false;
                            for attr in &field.attrs {
                                let config: Config = analyze_attributes(attr.to_token_stream());
                                if config.func_type.is_debug() && config.skip {
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
                    let expanded: TokenStream2 = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_tuple(#struct_name_str)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    };
                    TokenStream::from(expanded)
                }
                syn::Fields::Unit => {
                    let struct_name_str: String = name.to_string();
                    let expanded: TokenStream2 = quote! {
                        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(#struct_name_str).finish()
                            }
                        }
                    };
                    TokenStream::from(expanded)
                }
            }
        }
        Data::Enum(data_enum) => {
            let variants: Vec<TokenStream2> = data_enum
                .variants
                .iter()
                .map(|variant: &syn::Variant| {
                    let variant_name: &Ident = &variant.ident;
                    let variant_name_str: String = variant_name.to_string();
                    match &variant.fields {
                        syn::Fields::Named(fields_named) => {
                            let field_patterns: Vec<TokenStream2> = fields_named
                                .named
                                .iter()
                                .map(|field: &Field| {
                                    let field_name: &Ident = field.ident.as_ref().unwrap();
                                    quote! { #field_name }
                                })
                                .collect();
                            let debug_fields: Vec<TokenStream2> = fields_named
                                .named
                                .iter()
                                .filter_map(|field: &Field| {
                                    let field_name: &Ident = field.ident.as_ref()?;
                                    let mut should_skip: bool = false;
                                    for attr in &field.attrs {
                                        let config: Config =
                                            analyze_attributes(attr.to_token_stream());
                                        if config.func_type.is_debug() && config.skip {
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
                            let field_patterns: Vec<TokenStream2> = fields_unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, _): (usize, &Field)| {
                                    let field_name: Ident = format_ident!("field_{}", i);
                                    quote! { #field_name }
                                })
                                .collect();
                            let debug_fields: Vec<TokenStream2> = fields_unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .filter_map(|(i, field): (usize, &Field)| {
                                    let mut should_skip: bool = false;
                                    for attr in &field.attrs {
                                        let config: Config =
                                            analyze_attributes(attr.to_token_stream());
                                        if config.func_type.is_debug() && config.skip {
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
            let expanded: TokenStream2 = quote! {
                impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variants)*
                        }
                    }
                }
            };
            TokenStream::from(expanded)
        }
        Data::Union(_) => {
            panic!("Debug derive is not supported for unions")
        }
    }
}
