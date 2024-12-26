use super::constant::{GET, PRIVATE, PUBLIC, PUBLIC_CRATE, PUBLIC_SUPER, SET, SKIP};
use crate::{cfg::r#type::Cfg, func::r#type::FuncType, visibility::r#type::Visibility};
use proc_macro2::{TokenStream as NewTokenStream, TokenTree as NewTokenTree};

/// Parses the provided token stream and modifies the given configuration.
///
/// # Parameters
/// - `tokens`: A `NewTokenStream` containing the tokens to be parsed.
/// - `cfg`: A mutable reference to the `Cfg` structure that will be modified based on the parsed tokens.
///
/// # Returns
/// - The function does not return a value. It modifies the provided `cfg` in place.
pub fn parse_tokens(tokens: NewTokenStream, cfg: &mut Cfg) {
    for token in tokens {
        match token {
            NewTokenTree::Ident(ident) => {
                let ident_str: String = ident.to_string();
                if ident_str == GET || ident_str == SET {
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
            NewTokenTree::Group(group) => {
                parse_tokens(group.stream(), cfg);
            }
            _ => {}
        }
    }
}

/// Analyzes the given token stream and returns a configuration based on the attributes found.
///
/// # Parameters
/// - `tokens`: A `NewTokenStream` containing the tokens representing the attributes to be analyzed.
///
/// # Returns
/// - A `Cfg` structure representing the parsed configuration based on the attributes in the token stream.
pub fn analyze_attributes(tokens: NewTokenStream) -> Cfg {
    let mut cfg: Cfg = Cfg::default();
    parse_tokens(tokens, &mut cfg);
    cfg
}
