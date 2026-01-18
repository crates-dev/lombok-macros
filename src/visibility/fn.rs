use crate::*;

/// Parses the derive macro attributes to extract visibility information for the New constructor.
///
/// This function analyzes the arguments passed to the `#[derive(New(...))]` macro
/// to determine the desired visibility of the generated constructor function.
/// It supports various visibility modifiers like `pub`, `pub(crate)`, `pub(super)`, etc.
///
/// # Arguments
/// - `input` - The derive input to analyze for visibility attributes
///
/// # Returns
/// - The parsed visibility for the constructor, defaults to Public if not specified
pub(crate) fn parse_new_visibility(input: &DeriveInput) -> Visibility {
    for attr in &input.attrs {
        if attr.path().is_ident(NEW) {
            if let Ok(meta_list) = attr.meta.require_list() {
                let tokens: TokenStream2 = meta_list.tokens.clone();
                let mut visibility: Visibility = Visibility::Public;
                let mut iter: IntoIter = tokens.into_iter();
                while let Some(token) = iter.next() {
                    match token {
                        TokenTree2::Ident(ident) => {
                            if let Ok(parsed_visibility) = ident.to_string().parse::<Visibility>() {
                                match parsed_visibility {
                                    Visibility::Public => {
                                        if let Some(TokenTree2::Group(group)) = iter.next() {
                                            if group.delimiter() == Delimiter::Parenthesis {
                                                visibility = group.stream().to_string().parse::<Visibility>().expect("Failed to parse visibility from group tokens");
                                            }
                                        } else {
                                            visibility = Visibility::Public;
                                        }
                                        break;
                                    }
                                    Visibility::Private => {
                                        visibility = Visibility::Private;
                                        break;
                                    }
                                    _ => {
                                        visibility = parsed_visibility;
                                        break;
                                    }
                                }
                            }
                        }
                        TokenTree2::Group(group) => {
                            for sub_token in group.stream().into_iter() {
                                if let TokenTree2::Ident(ident) = sub_token {
                                    visibility = ident
                                        .to_string()
                                        .parse::<Visibility>()
                                        .expect("Failed to parse visibility from group tokens");
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return visibility;
            }
        }
    }
    Visibility::Public
}
