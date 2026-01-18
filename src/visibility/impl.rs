use crate::*;

impl Visibility {
    /// Converts the `Visibility` enum variant into a token stream representation.
    ///
    /// # Arguments
    /// - `self` - The reference to the `Visibility` instance being converted.
    ///
    /// # Returns
    /// - `TokenStream2` - representing the corresponding visibility modifier in Rust syntax.
    pub(crate) fn to_token_stream(self) -> TokenStream2 {
        match self {
            Visibility::Public => quote! { pub },
            Visibility::PublicCrate => quote! { pub(crate) },
            Visibility::PublicSuper => quote! { pub(super) },
            Visibility::Private => quote! {},
        }
    }
}

/// Implementation of Display trait for Visibility enum.
/// This allows Visibility variants to be formatted as strings using {:?} or {} formatting.
impl Display for Visibility {
    /// Formats the `Visibility` enum variant into its string representation.
    ///
    /// # Arguments
    /// - `self` - The reference to the `Visibility` instance being formatted.
    /// - `Formatter<'_>` - The formatter to write the string representation to.
    ///
    /// # Returns
    /// - `Result` - indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Visibility::Public => PUB,
            Visibility::PublicCrate => PUB_CRATE,
            Visibility::PublicSuper => PUB_SUPER,
            Visibility::Private => PRIVATE,
        };
        write!(f, "{s}")
    }
}

/// Implementation of FromStr trait for Visibility enum.
/// This allows parsing string representations back into Visibility variants.
impl std::str::FromStr for Visibility {
    type Err = String;

    /// Parses a string into a `Visibility` enum variant.
    ///
    /// # Arguments
    /// - `&str` - The string slice to parse into a Visibility variant.
    ///
    /// # Returns
    /// - `Result` - containing the parsed Visibility variant or an error message.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PUB => Ok(Visibility::Public),
            CRATE | PUB_CRATE => Ok(Visibility::PublicCrate),
            SUPER | PUB_SUPER => Ok(Visibility::PublicSuper),
            PRIVATE => Ok(Visibility::Private),
            _ => Err(format!("Unknown visibility modifier: {s}")),
        }
    }
}
