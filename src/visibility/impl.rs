use super::r#type::Visibility;
use proc_macro2::TokenStream as NewTokenStream;
use quote::quote;

impl Visibility {
    /// Converts the `Visibility` enum variant into a token stream representation.
    ///
    /// # Parameters
    /// - `self`: The reference to the `Visibility` instance being converted.
    ///
    /// # Returns
    /// - A `NewTokenStream` representing the corresponding visibility modifier in Rust syntax.
    #[inline]
    pub fn to_token_stream(&self) -> NewTokenStream {
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
    #[inline]
    pub fn is_public(&self) -> bool {
        *self == Self::Public
    }
}
