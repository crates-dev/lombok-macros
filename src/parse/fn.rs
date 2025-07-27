use crate::*;

/// Parses the provided token stream and modifies the given configuration.
///
/// # Parameters
/// - `tokens`: A `TokenStream2` containing the tokens to be parsed.
/// - `cfg`: A mutable reference to the `Cfg` structure that will be modified based on the parsed tokens.
///
/// # Returns
/// - The function does not return a value. It modifies the provided `cfg` in place.
pub fn parse_tokens(tokens: TokenStream2, cfg: &mut Cfg) {
    for token in tokens {
        match token {
            TokenTree2::Ident(ident) => {
                let ident_str: String = ident.to_string();
                if FuncType::is_known(&ident_str) {
                    if cfg.func_type.is_unknown() {
                        cfg.func_type = ident_str.parse::<FuncType>().unwrap_or_default();
                    }
                } else if ident_str == SKIP {
                    cfg.skip = true;
                } else if ident_str == PUBLIC {
                    cfg.visibility = Visibility::Public;
                } else if ident_str == PRIVATE {
                    cfg.visibility = Visibility::Private;
                } else if ident_str == PUBLIC_CRATE && cfg.visibility.is_public() {
                    cfg.visibility = Visibility::PublicCrate;
                } else if ident_str == PUBLIC_SUPER && cfg.visibility.is_public() {
                    cfg.visibility = Visibility::PublicSuper;
                }
            }
            TokenTree2::Group(group) => {
                parse_tokens(group.stream(), cfg);
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
/// - A `Cfg` structure representing the parsed configuration based on the attributes in the token stream.
pub fn analyze_attributes(tokens: TokenStream2) -> Cfg {
    let mut cfg: Cfg = Cfg::default();
    parse_tokens(tokens, &mut cfg);
    cfg
}
