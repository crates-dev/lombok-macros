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
    let clean_attr: String = if let Some(stripped) = attr_str.strip_prefix(RAW_IDENT_PREFIX) {
        stripped.to_owned()
    } else {
        attr_str.to_owned()
    };
    clean_attr
}

/// Checks if a type is an Option<T> type.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - true if the type is Option<T>, false otherwise.
fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == OPTION_TYPE
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a type is a Result<T, E> type.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - true if the type is Result<T, E>, false otherwise.
fn is_result_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == RESULT_TYPE
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a type is a Box<T> type.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - true if the type is Box<T>, false otherwise.
fn is_box_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Box"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a type is an Rc<T> type.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - true if the type is Rc<T>, false otherwise.
fn is_rc_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Rc"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if a type is an Arc<T> type.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - true if the type is Arc<T>, false otherwise.
fn is_arc_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Arc"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Generates the appropriate parameter type based on the field type.
///
/// # Arguments
///
/// - `&Type` - The original field type.
/// - `param_type_override` - Optional custom parameter type from attribute specification.
///
/// # Returns
///
/// - `TokenStream2` - The generated parameter type as tokens.
fn generate_param_type(
    field_type: &Type,
    param_type_override: Option<&TokenStream2>,
) -> TokenStream2 {
    if let Some(override_type) = param_type_override {
        let type_str: String = override_type.to_string();
        let param_type: ParameterType = ParameterType::from(type_str.as_str());
        match param_type {
            ParameterType::AsRef => {
                quote! { impl #override_type }
            }
            ParameterType::Into => {
                quote! { impl #override_type }
            }
            ParameterType::AsMut => {
                quote! { impl #override_type }
            }
            ParameterType::Deref => {
                quote! { impl #override_type }
            }
            ParameterType::Direct => override_type.clone(),
            ParameterType::Custom(custom_tokens) => {
                quote! { impl #custom_tokens }
            }
        }
    } else {
        quote! { #field_type }
    }
}

/// Generates the appropriate assignment expression based on the parameter type.
///
/// # Arguments
///
/// - `ident` - The field identifier to assign to.
/// - `param_type_override` - Optional custom parameter type from attribute specification.
///
/// # Returns
///
/// - `TokenStream2` - The generated assignment expression.
fn generate_assignment(
    field_ident: &proc_macro2::Ident,
    param_type_override: Option<&TokenStream2>,
) -> TokenStream2 {
    if let Some(override_type) = param_type_override {
        let type_str: String = override_type.to_string();
        let param_type: ParameterType = ParameterType::from(type_str.as_str());
        match param_type {
            ParameterType::AsRef => {
                quote! { self.#field_ident = val.as_ref().to_owned(); }
            }
            ParameterType::Into
            | ParameterType::AsMut
            | ParameterType::Deref
            | ParameterType::Custom(_) => {
                quote! { self.#field_ident = val.into(); }
            }
            ParameterType::Direct => {
                quote! { self.#field_ident = val; }
            }
        }
    } else {
        quote! { self.#field_ident = val; }
    }
}

/// Generates the appropriate assignment expression for tuple structs.
///
/// # Arguments
///
/// - `index` - The tuple field index to assign to.
/// - `param_type_override` - Optional custom parameter type from attribute specification.
///
/// # Returns
///
/// - `TokenStream2` - The generated assignment expression.
fn generate_assignment_tuple(
    field_index: &Index,
    param_type_override: Option<&TokenStream2>,
) -> TokenStream2 {
    if let Some(override_type) = param_type_override {
        let type_str: String = override_type.to_string();
        let param_type: ParameterType = ParameterType::from(type_str.as_str());
        match param_type {
            ParameterType::AsRef => {
                quote! { self.#field_index = val.as_ref().to_owned(); }
            }
            ParameterType::Into
            | ParameterType::AsMut
            | ParameterType::Deref
            | ParameterType::Custom(_) => {
                quote! { self.#field_index = val.into(); }
            }
            ParameterType::Direct => {
                quote! { self.#field_index = val; }
            }
        }
    } else {
        quote! { self.#field_index = val; }
    }
}

/// Generates the appropriate return type based on the field type and return type strategy.
///
/// # Arguments
///
/// - `&Type` - The original field type.
/// - `ReturnType` - The return type strategy to apply.
///
/// # Returns
///
/// - `TokenStream2` - The generated return type as tokens.
fn generate_return_type(field_type: &Type, return_type: ReturnType) -> TokenStream2 {
    match return_type {
        ReturnType::Reference => {
            quote! { &#field_type }
        }
        ReturnType::Clone | ReturnType::Copy => {
            quote! { #field_type }
        }
        ReturnType::Deref => {
            if is_option_type(field_type)
                || is_result_type(field_type)
                || is_box_type(field_type)
                || is_rc_type(field_type)
                || is_arc_type(field_type)
            {
                if let Type::Path(type_path) = field_type {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                quote! { #ty }
                            } else {
                                quote! { #field_type }
                            }
                        } else {
                            quote! { #field_type }
                        }
                    } else {
                        quote! { #field_type }
                    }
                } else {
                    quote! { #field_type }
                }
            } else {
                quote! { #field_type }
            }
        }
        ReturnType::Default => {
            if is_option_type(field_type) || is_result_type(field_type) {
                if let Type::Path(type_path) = field_type {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                quote! { #ty }
                            } else {
                                quote! { #field_type }
                            }
                        } else {
                            quote! { #field_type }
                        }
                    } else {
                        quote! { #field_type }
                    }
                } else {
                    quote! { #field_type }
                }
            } else {
                quote! { &#field_type }
            }
        }
    }
}

/// Generates a getter function for named struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident` - The name of the getter function.
/// - `&Ident` - The name of the field.
/// - `&Type` - The type of the field.
/// - `ReturnType` - The return type of the getter function.
///
/// # Returns
///
/// - `TokenStream2` - The generated getter function.
fn build_named_get_quote(
    need_getter: bool,
    vis: TokenStream2,
    get_name: &Ident,
    attr_name_ident: &Ident,
    attr_ty: &Type,
    return_type: ReturnType,
) -> TokenStream2 {
    if !need_getter {
        return quote! {};
    }
    let return_ty: TokenStream2 = generate_return_type(attr_ty, return_type);
    match return_type {
        ReturnType::Reference => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                &self.#attr_name_ident
            }
        },
        ReturnType::Clone => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                self.#attr_name_ident.clone()
            }
        },
        ReturnType::Copy => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                self.#attr_name_ident
            }
        },
        ReturnType::Deref => {
            if is_option_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        match &self.#attr_name_ident {
                            Some(value) => *value,
                            None => panic!("Attempted to dereference None value for field '{}'", stringify!(#attr_name_ident)),
                        }
                    }
                }
            } else if is_result_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        match &self.#attr_name_ident {
                            Ok(value) => (*value).clone(),
                            Err(err) => panic!("Failed to dereference Result for field '{}': {:?}", stringify!(#attr_name_ident), err),
                        }
                    }
                }
            } else if is_box_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        *self.#attr_name_ident
                    }
                }
            } else if is_rc_type(attr_ty) || is_arc_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        std::clone::Clone::clone(&*self.#attr_name_ident)
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        *self.#attr_name_ident
                    }
                }
            }
        }
        ReturnType::Default => {
            if is_option_type(attr_ty) || is_result_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        self.#attr_name_ident.clone().unwrap()
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        &self.#attr_name_ident
                    }
                }
            }
        }
    }
}

/// Generates a try getter function for named struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a try getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident` - The name of the try getter function.
/// - `&Ident` - The name of the field.
/// - `&Type` - The type of the field.
///
/// # Returns
///
/// - `TokenStream2` - The generated try getter function.
fn build_named_try_get_quote(
    need_getter: bool,
    vis: TokenStream2,
    get_name: &Ident,
    attr_name_ident: &Ident,
    attr_ty: &Type,
) -> TokenStream2 {
    if need_getter && (is_option_type(attr_ty) || is_result_type(attr_ty)) {
        let try_get_name: Ident = format_ident!("{}{}", TRY_GET_METHOD_PREFIX, get_name);
        quote! {
            #[inline(always)]
            #vis fn #try_get_name(&self) -> &#attr_ty {
                &self.#attr_name_ident
            }
        }
    } else {
        quote! {}
    }
}

/// Generates a mutable getter function for named struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a mutable getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident` - The name of the mutable getter function.
/// - `&Ident` - The name of the field.
/// - `&Type` - The type of the field.
///
/// # Returns
///
/// - `TokenStream2` - The generated mutable getter function.
fn build_named_get_mut_quote(
    need_getter_mut: bool,
    vis: TokenStream2,
    get_mut_name: &Ident,
    attr_name_ident: &Ident,
    attr_ty: &Type,
) -> TokenStream2 {
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
fn build_named_set_quote(
    need_setter: bool,
    vis: TokenStream2,
    set_name: &Ident,
    attr_name_ident: &Ident,
    attr_ty: &Type,
    param_type_override: Option<&TokenStream2>,
) -> TokenStream2 {
    if need_setter {
        let param_type: TokenStream2 = generate_param_type(attr_ty, param_type_override);
        let assignment: TokenStream2 = generate_assignment(attr_name_ident, param_type_override);
        quote! {
            #[inline(always)]
            #vis fn #set_name(&mut self, val: #param_type) -> &mut Self {
                #assignment
                self
            }
        }
    } else {
        quote! {}
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
    let attr_name_ident: &Ident = field.ident.as_ref().expect(FIELD_SHOULD_HAVE_A_NAME);
    let attr_ty: &Type = &field.ty;
    let clean_attr_name: String = get_clean_attr_name(&attr_name_ident.to_string());
    let get_name: Ident = format_ident!("{}{}", GET_METHOD_PREFIX, clean_attr_name);
    let get_mut_name: Ident = format_ident!("{}{}", GET_MUT_METHOD_PREFIX, clean_attr_name);
    let set_name: Ident = format_ident!("{}{}", SET_METHOD_PREFIX, clean_attr_name);
    let mut generated: TokenStream2 = quote! {};
    let mut config_map: HashMap<String, Vec<Config>> = HashMap::new();
    for attr in &field.attrs {
        let config: Config = analyze_attributes(attr.to_token_stream());
        let name: String = attr_name_ident.to_string();
        config_map.entry(name).or_default().push(config);
    }
    let mut has_add_get: bool = false;
    let mut has_add_get_mut: bool = false;
    let mut has_add_set: bool = false;
    for config_list in config_map.values() {
        for config in config_list {
            if has_add_get && has_add_get_mut && has_add_set {
                break;
            }
            if config.skip && config.func_type.is_unknown() {
                continue;
            }
            let vis: TokenStream2 = config.visibility.to_token_stream();
            if config.func_type.is_get() && !config.skip && !has_add_get {
                generated.extend(build_named_get_quote(
                    need_getter,
                    vis.clone(),
                    &get_name,
                    attr_name_ident,
                    attr_ty,
                    config.return_type,
                ));
                generated.extend(build_named_try_get_quote(
                    need_getter,
                    vis.clone(),
                    &get_name,
                    attr_name_ident,
                    attr_ty,
                ));
                has_add_get = true;
            }
            if config.func_type.is_get_mut() && !config.skip && !has_add_get_mut {
                generated.extend(build_named_get_mut_quote(
                    need_getter_mut,
                    vis.clone(),
                    &get_mut_name,
                    attr_name_ident,
                    attr_ty,
                ));
                has_add_get_mut = true;
            }
            if config.func_type.is_set() && !config.skip && !has_add_set {
                generated.extend(build_named_set_quote(
                    need_setter,
                    vis.clone(),
                    &set_name,
                    attr_name_ident,
                    attr_ty,
                    config.param_type_override.as_ref(),
                ));
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_get_mut || !has_add_set {
        let config: Config = Config::default();
        let vis: TokenStream2 = config.visibility.to_token_stream();
        if !has_add_get {
            generated.extend(build_named_get_quote(
                need_getter,
                vis.clone(),
                &get_name,
                attr_name_ident,
                attr_ty,
                config.return_type,
            ));
            generated.extend(build_named_try_get_quote(
                need_getter,
                vis.clone(),
                &get_name,
                attr_name_ident,
                attr_ty,
            ));
        }
        if !has_add_get_mut {
            generated.extend(build_named_get_mut_quote(
                need_getter_mut,
                vis.clone(),
                &get_mut_name,
                attr_name_ident,
                attr_ty,
            ));
        }
        if !has_add_set {
            generated.extend(build_named_set_quote(
                need_setter,
                vis.clone(),
                &set_name,
                attr_name_ident,
                attr_ty,
                None,
            ));
        }
    }
    generated
}

/// Generates a getter function for tuple struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident,` - The name of the getter function.
/// - `&Index,` - The index of the field in the tuple struct.
/// - `&Type` - The type of the field.
/// - `ReturnType` - The return type of the getter function.
///
/// # Returns
///
/// - `TokenStream2` - The generated getter function.
fn build_tuple_get_quote(
    need_getter: bool,
    vis: TokenStream2,
    get_name: &Ident,
    field_index: &Index,
    attr_ty: &Type,
    return_type: ReturnType,
) -> TokenStream2 {
    if !need_getter {
        return quote! {};
    }
    let return_ty: TokenStream2 = generate_return_type(attr_ty, return_type);
    match return_type {
        ReturnType::Reference => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                &self.#field_index
            }
        },
        ReturnType::Clone => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                self.#field_index.clone()
            }
        },
        ReturnType::Copy => quote! {
            #[inline(always)]
            #vis fn #get_name(&self) -> #return_ty {
                self.#field_index
            }
        },
        ReturnType::Deref => {
            if is_option_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        match &self.#field_index {
                            Some(value) => *value,
                            None => panic!("Cannot dereference None value"),
                        }
                    }
                }
            } else if is_result_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        match &self.#field_index {
                            Ok(value) => (*value).clone(),
                            Err(e) => panic!("Cannot dereference Err value: {:?}", e),
                        }
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        self.#field_index
                    }
                }
            }
        }
        ReturnType::Default => {
            if is_option_type(attr_ty) || is_result_type(attr_ty) {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        self.#field_index.clone().unwrap()
                    }
                }
            } else {
                quote! {
                    #[inline(always)]
                    #vis fn #get_name(&self) -> #return_ty {
                        &self.#field_index
                    }
                }
            }
        }
    }
}

/// Generates a try getter function for tuple struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a try getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident` - The name of the try getter function.
/// - `&Index` - The index of the field in the tuple struct.
/// - `&Type` - The type of the field.
///
/// # Returns
///
/// - `TokenStream2` - The generated try getter function.
fn build_tuple_try_get_quote(
    need_getter: bool,
    vis: TokenStream2,
    get_name: &Ident,
    field_index: &Index,
    attr_ty: &Type,
) -> TokenStream2 {
    if need_getter && (is_option_type(attr_ty) || is_result_type(attr_ty)) {
        let try_get_name: Ident = format_ident!("{}{}", TRY_GET_METHOD_PREFIX, get_name);
        quote! {
            #[inline(always)]
            #vis fn #try_get_name(&self) -> &#attr_ty {
                &self.#field_index
            }
        }
    } else {
        quote! {}
    }
}

/// Generates a mutable getter function for tuple struct fields.
///
/// # Arguments
///
/// - `bool` - Whether to generate a mutable getter function.
/// - `TokenStream2` - The visibility of the function.
/// - `&Ident` - The name of the mutable getter function.
/// - `&Index` - The index of the field in the tuple struct.
/// - `&Type` - The type of the field.
///
/// # Returns
///
/// - `TokenStream2` - The generated mutable getter function.
fn build_tuple_get_mut_quote(
    need_getter_mut: bool,
    vis: TokenStream2,
    get_mut_name: &Ident,
    field_index: &Index,
    attr_ty: &Type,
) -> TokenStream2 {
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
fn build_tuple_set_quote(
    need_setter: bool,
    vis: TokenStream2,
    set_name: &Ident,
    field_index: &Index,
    attr_ty: &Type,
    param_type_override: Option<&TokenStream2>,
) -> TokenStream2 {
    if need_setter {
        let param_type: TokenStream2 = generate_param_type(attr_ty, param_type_override);
        let assignment: TokenStream2 = generate_assignment_tuple(field_index, param_type_override);
        quote! {
            #[inline(always)]
            #vis fn #set_name(&mut self, val: #param_type) -> &mut Self {
                #assignment
                self
            }
        }
    } else {
        quote! {}
    }
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
    let field_index: Index = Index::from(index);
    let mut generated: TokenStream2 = quote! {};
    let mut config_map: HashMap<String, Vec<Config>> = HashMap::new();
    for attr in &field.attrs {
        let config: Config = analyze_attributes(attr.to_token_stream());
        let name: String = index.to_string();
        config_map.entry(name).or_default().push(config);
    }
    let mut has_add_get: bool = false;
    let mut has_add_get_mut: bool = false;
    let mut has_add_set: bool = false;
    for config_list in config_map.values() {
        for config in config_list {
            if has_add_get && has_add_get_mut && has_add_set {
                break;
            }
            if config.skip && config.func_type.is_unknown() {
                continue;
            }
            let vis: TokenStream2 = config.visibility.to_token_stream();
            if config.func_type.is_get() && !config.skip && !has_add_get {
                generated.extend(build_tuple_get_quote(
                    need_getter,
                    vis.clone(),
                    &get_name,
                    &field_index,
                    attr_ty,
                    config.return_type,
                ));
                generated.extend(build_tuple_try_get_quote(
                    need_getter,
                    vis.clone(),
                    &get_name,
                    &field_index,
                    attr_ty,
                ));
                has_add_get = true;
            }
            if config.func_type.is_get_mut() && !config.skip && !has_add_get_mut {
                generated.extend(build_tuple_get_mut_quote(
                    need_getter_mut,
                    vis.clone(),
                    &get_mut_name,
                    &field_index,
                    attr_ty,
                ));
                has_add_get_mut = true;
            }
            if config.func_type.is_set() && !config.skip && !has_add_set {
                generated.extend(build_tuple_set_quote(
                    need_setter,
                    vis.clone(),
                    &set_name,
                    &field_index,
                    attr_ty,
                    config.param_type_override.as_ref(),
                ));
                has_add_set = true;
            }
        }
    }
    if !has_add_get || !has_add_get_mut || !has_add_set {
        let config: Config = Config::default();
        let vis: TokenStream2 = config.visibility.to_token_stream();
        if !has_add_get {
            generated.extend(build_tuple_get_quote(
                need_getter,
                vis.clone(),
                &get_name,
                &field_index,
                attr_ty,
                config.return_type,
            ));
            generated.extend(build_tuple_try_get_quote(
                need_getter,
                vis.clone(),
                &get_name,
                &field_index,
                attr_ty,
            ));
        }
        if !has_add_get_mut {
            generated.extend(build_tuple_get_mut_quote(
                need_getter_mut,
                vis.clone(),
                &get_mut_name,
                &field_index,
                attr_ty,
            ));
        }
        if !has_add_set {
            generated.extend(build_tuple_set_quote(
                need_setter,
                vis.clone(),
                &set_name,
                &field_index,
                attr_ty,
                None,
            ));
        }
    }
    generated
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
            Fields::Named(_) => s
                .fields
                .iter()
                .map(|field| {
                    generate_getter_setter(field, None, need_getter, need_getter_mut, need_setter)
                })
                .collect::<Vec<_>>(),
            Fields::Unnamed(_) => s
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
            Fields::Unit => Vec::new(),
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
    let generics: &Generics = &input.generics;
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
    let generics: &Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    match &input.data {
        Data::Struct(data_struct) => {
            let fields: &Fields = &data_struct.fields;
            match fields {
                Fields::Named(_) => {
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
                Fields::Unnamed(_) => {
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
                                let field_index: Index = Index::from(i);
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
                Fields::Unit => {
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
                .map(|variant: &Variant| {
                    let variant_name: &Ident = &variant.ident;
                    let variant_name_str: String = variant_name.to_string();
                    match &variant.fields {
                        Fields::Named(fields_named) => {
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
                        Fields::Unnamed(fields_unnamed) => {
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
                        Fields::Unit => {
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

/// Checks if a field should be skipped for New constructor generation.
///
/// # Arguments
///
/// - `&Field` - The field structure to analyze for skip attribute.
///
/// # Returns
///
/// - `bool` - True if the field should be skipped, false otherwise.
fn should_skip_field_for_new(field: &Field) -> bool {
    let mut should_skip: bool = false;
    for attr in &field.attrs {
        let config: Config = analyze_attributes(attr.to_token_stream());
        if config.func_type.is_new() && config.skip {
            should_skip = true;
            break;
        }
    }
    should_skip
}

/// Generates a constructor function for named struct fields, excluding skipped fields.
///
/// # Arguments
///
/// - `&Field` - The field structure to analyze for constructor generation.
///
/// # Returns
///
/// - `Option<(Ident, Type)>` - The field name and type if not skipped, None if skipped.
fn analyze_named_field_for_new(field: &Field) -> Option<(Ident, Type)> {
    if should_skip_field_for_new(field) {
        return None;
    }
    let field_name: &Ident = field.ident.as_ref()?;
    let field_type: &Type = &field.ty;
    Some((field_name.clone(), field_type.clone()))
}

/// Generates a constructor function for tuple struct fields, excluding skipped fields.
///
/// # Arguments
///
/// - `&Field` - The field structure to analyze for constructor generation.
/// - `usize` - The index of the field in the tuple.
///
/// # Returns
///
/// - `Option<(Ident, Type)>` - The generated parameter name and field type if not skipped, None if skipped.
fn analyze_tuple_field_for_new(field: &Field, index: usize) -> Option<(Ident, Type)> {
    if should_skip_field_for_new(field) {
        return None;
    }
    let field_type: &Type = &field.ty;
    let param_name: Ident = format_ident!("field_{}", index);
    Some((param_name, field_type.clone()))
}

/// Generates a constructor function for a struct with the specified visibility.
///
/// # Arguments
///
/// - `&DeriveInput` - The derive input representing the struct.
/// - `Visibility` - The visibility for the generated constructor function.
///
/// # Returns
///
/// - `TokenStream` - The generated constructor implementation.
pub(crate) fn inner_new_constructor(input: &DeriveInput, visibility: Visibility) -> TokenStream {
    let name: &Ident = &input.ident;
    let generics: &Generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields_info: Vec<(Ident, Type)> = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .filter_map(analyze_named_field_for_new)
                .collect(),
            Fields::Unnamed(unnamed_fields) => unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .filter_map(|(index, field)| analyze_tuple_field_for_new(field, index))
                .collect(),
            Fields::Unit => Vec::new(),
        },
        _ => panic!("{}", UNSUPPORTED_NEW_DERIVE),
    };
    let tuple_field_mapping: Vec<(Index, Ident)> = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Unnamed(unnamed_fields) => unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .filter_map(|(index, field)| {
                    if !should_skip_field_for_new(field) {
                        let param_name: Ident = format_ident!("field_{}", index);
                        Some((Index::from(index), param_name))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };
    let params: Vec<TokenStream2> = fields_info
        .iter()
        .map(|(field_name, field_type)| {
            quote! { #field_name: #field_type }
        })
        .collect();
    let constructor_fields: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(_) => {
                let field_initializers: Vec<TokenStream2> = data_struct
                    .fields
                    .iter()
                    .filter_map(|field| {
                        let original_name: &Ident = field.ident.as_ref()?;
                        if !should_skip_field_for_new(field) {
                            Some(quote! { #original_name: #original_name })
                        } else {
                            Some(quote! { #original_name: Default::default() })
                        }
                    })
                    .collect();
                quote! { { #(#field_initializers),* } }
            }
            Fields::Unnamed(_) => {
                let field_initializers: Vec<TokenStream2> = data_struct
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        let field_index: Index = Index::from(index);
                        if !should_skip_field_for_new(field) {
                            if let Some((_, param_name)) = tuple_field_mapping
                                .iter()
                                .find(|(idx, _)| *idx == field_index)
                            {
                                quote! { #param_name }
                            } else {
                                quote! { Default::default() }
                            }
                        } else {
                            quote! { Default::default() }
                        }
                    })
                    .collect();
                quote! { ( #(#field_initializers),* ) }
            }
            Fields::Unit => quote! { {} },
        },
        _ => panic!("{}", UNSUPPORTED_NEW_DERIVE),
    };
    let vis_tokens: TokenStream2 = visibility.to_token_stream();
    let expanded: TokenStream2 = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline(always)]
            #vis_tokens fn new(#(#params),*) -> Self {
                Self #constructor_fields
            }
        }
    };
    expanded.into()
}
