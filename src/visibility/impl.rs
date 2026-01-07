use crate::*;

impl Visibility {
    /// Converts the `Visibility` enum variant into a token stream representation.
    ///
    /// # Parameters
    /// - `self`: The reference to the `Visibility` instance being converted.
    ///
    /// # Returns
    /// - A `TokenStream2` representing the corresponding visibility modifier in Rust syntax.
    pub(crate) fn to_token_stream(&self) -> TokenStream2 {
        match self {
            Visibility::Public => quote! { pub },
            Visibility::PublicCrate => quote! { pub(crate) },
            Visibility::PublicSuper => quote! { pub(super) },
            Visibility::Private => quote! {},
        }
    }

    /// Determines if the `Visibility` is public.
    ///
    /// # Parameters
    /// - `self`: The reference to the `Visibility` instance being checked.
    ///
    /// # Returns
    /// - `true` if the visibility is `Public`; otherwise, `false`.
    #[inline(always)]
    pub(crate) fn is_public(&self) -> bool {
        *self == Self::Public
    }
}

/// Implementation of Display trait for Visibility enum.
/// This allows Visibility variants to be formatted as strings using {:?} or {} formatting.
impl std::fmt::Display for Visibility {
    /// Formats the `Visibility` enum variant into its string representation.
    ///
    /// # Parameters
    /// - `self`: The reference to the `Visibility` instance being formatted.
    /// - `f`: The formatter to write the string representation to.
    ///
    /// # Returns
    /// - `Result` indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Visibility::Public => "pub",
            Visibility::PublicCrate => "pub(crate)",
            Visibility::PublicSuper => "pub(super)",
            Visibility::Private => "private",
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
    /// # Parameters
    /// - `s`: The string slice to parse into a Visibility variant.
    ///
    /// # Returns
    /// - `Result` containing the parsed Visibility variant or an error message.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pub" => Ok(Visibility::Public),
            "crate" | "pub(crate)" => Ok(Visibility::PublicCrate),
            "super" | "pub(super)" => Ok(Visibility::PublicSuper),
            "private" => Ok(Visibility::Private),
            _ => Err(format!("Unknown visibility modifier: {s}")),
        }
    }
}
