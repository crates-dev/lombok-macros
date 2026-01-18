use crate::*;

/// Parses the provided token stream and modifies the given configuration.
///
/// # Arguments
/// - `tokens` - A `TokenStream2` containing the tokens to be parsed.
/// - `config` - A mutable reference to the `Config` structure that will be modified based on the parsed tokens.
///
/// # Returns
/// - The function does not return a value. It modifies the provided `config` in place.
pub(crate) fn parse_tokens(tokens: TokenStream2, config: &mut Config) {
    let mut tokens_iter: Peekable<IntoIter> = tokens.into_iter().peekable();
    while let Some(token) = tokens_iter.next() {
        match token {
            TokenTree2::Ident(ident) => {
                let ident_str: String = ident.to_string();
                if FuncType::is_known(&ident_str) {
                    if config.func_type.is_unknown() {
                        config.func_type = ident_str.parse::<FuncType>().unwrap_or_default();
                    }
                } else if ident_str == SKIP {
                    config.skip = true;
                } else if ident_str == PUB {
                    let mut lookahead: Peekable<IntoIter> = tokens_iter.clone();
                    if let Some(TokenTree2::Group(group)) = lookahead.next() {
                        if group.delimiter() == Delimiter::Parenthesis {
                            let group_content = group.stream().to_string();
                            if group_content == CRATE {
                                config.visibility = Visibility::PublicCrate;
                                tokens_iter.next();
                            } else if group_content == SUPER {
                                config.visibility = Visibility::PublicSuper;
                                tokens_iter.next();
                            }
                        }
                    } else {
                        config.visibility = Visibility::Public;
                    }
                } else if ident_str == PRIVATE {
                    config.visibility = Visibility::Private;
                } else if ident_str == CUSTOM_TYPE {
                    if let Some(TokenTree2::Group(group)) = tokens_iter.peek() {
                        if group.delimiter() == Delimiter::Parenthesis {
                            let type_group: TokenTree2 = tokens_iter.next().unwrap();
                            if let TokenTree2::Group(group) = type_group {
                                config.return_type = group
                                    .stream()
                                    .to_string()
                                    .parse::<ReturnType>()
                                    .unwrap_or_default();
                                config.param_type_override = Some(group.stream());
                            }
                        }
                    }
                }
            }
            TokenTree2::Group(group) => {
                parse_tokens(group.stream(), config);
            }
            _ => {}
        }
    }
}

/// Analyzes the given token stream and returns a configuration based on the attributes found.
///
/// # Arguments
/// - `tokens` - A `TokenStream2` containing the tokens representing the attributes to be analyzed.
///
/// # Returns
/// - A `Config` structure representing the parsed configuration based on the attributes in the token stream.
pub(crate) fn analyze_attributes(tokens: TokenStream2) -> Config {
    let mut config: Config = Config::default();
    parse_tokens(tokens, &mut config);
    config
}
