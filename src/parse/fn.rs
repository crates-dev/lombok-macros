use crate::*;

/// Parses the provided token stream and modifies the given configuration.
///
/// # Parameters
/// - `tokens`: A `TokenStream2` containing the tokens to be parsed.
/// - `config`: A mutable reference to the `Config` structure that will be modified based on the parsed tokens.
///
/// # Returns
/// - The function does not return a value. It modifies the provided `config` in place.
pub(crate) fn parse_tokens(tokens: TokenStream2, config: &mut Config) {
    for token in tokens {
        match token {
            TokenTree2::Ident(ident) => {
                let ident_str: String = ident.to_string();
                if FuncType::is_known(&ident_str) {
                    if config.func_type.is_unknown() {
                        config.func_type = ident_str.parse::<FuncType>().unwrap_or_default();
                    }
                } else if ident_str == SKIP {
                    config.skip = true;
                } else if ident_str == PUBLIC {
                    config.visibility = Visibility::Public;
                } else if ident_str == PRIVATE {
                    config.visibility = Visibility::Private;
                } else if ident_str == PUBLIC_CRATE && config.visibility.is_public() {
                    config.visibility = Visibility::PublicCrate;
                } else if ident_str == PUBLIC_SUPER && config.visibility.is_public() {
                    config.visibility = Visibility::PublicSuper;
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
/// # Parameters
/// - `tokens`: A `TokenStream2` containing the tokens representing the attributes to be analyzed.
///
/// # Returns
/// - A `Config` structure representing the parsed configuration based on the attributes in the token stream.
pub(crate) fn analyze_attributes(tokens: TokenStream2) -> Config {
    let mut config: Config = Config::default();
    parse_tokens(tokens, &mut config);
    config
}
