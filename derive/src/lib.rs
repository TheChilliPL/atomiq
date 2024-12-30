extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

#[proc_macro_derive(Atomizable)]
pub fn derive_atomizable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let (field_type, pack_line, unpack_line) = if let syn::Data::Struct(ref data) = input.data {
        let field = if let syn::Fields::Named(ref fields) = data.fields {
            if fields.named.len() != 1 {
                return TokenStream::from(quote_spanned! { fields.span() =>
                    compile_error!("Atomizable can only be derived for structs with a single field.");
                });
            }
            fields.named.get(0).unwrap()
        } else if let syn::Fields::Unnamed(ref fields) = data.fields {
            if fields.unnamed.len() != 1 {
                return TokenStream::from(quote_spanned! { fields.span() =>
                    compile_error!("Atomizable can only be derived for structs with a single field.");
                });
            }
            fields.unnamed.get(0).unwrap()
        } else {
            return TokenStream::from(quote! {
                compile_error!("Atomizable can only be derived for structs with a single field.");
            });
        };

        let field_name = field
            .ident
            .as_ref();

        if let Some(ref field_name) = field_name {
            (field.ty.to_token_stream(), quote! { self.#field_name }, quote! { #name { #field_name: atom } })
        } else {
            (field.ty.to_token_stream(), quote! { self.0 }, quote! { #name(atom) })
        }
    } else if let syn::Data::Enum(ref data) = input.data {
        let repr_attr = input.attrs.iter().find(|attr| {
            attr.meta.path().is_ident("repr")
        });

        if repr_attr.is_none() {
            return TokenStream::from(quote! {
                compile_error!(
                    "Atomizable can only be derived for enums with an explicit repr attribute."
                );
            });
        }

        let repr_ident: Ident = repr_attr.unwrap().parse_args().unwrap();

        let fielded_variants = data.variants.iter().filter(|variant| !variant.fields.is_empty())
            .map(|variant| {
                quote_spanned! {variant.fields.span()=>
                    compile_error!(
                        "Atomizable can only be derived for enums with only unit variants."
                    );
                }
            })
            .collect::<Vec<_>>();
        if !fielded_variants.is_empty() {
            return TokenStream::from(quote! {
                #(#fielded_variants)*
            });
        }

        (
            repr_ident.to_token_stream(),
            quote!(self as #repr_ident),
            quote!(unsafe { core::mem::transmute(atom) }),
        )
    } else {
        return TokenStream::from(quote! {
            compile_error!("Atomizable can only be derived for structs and enums.");
        });
    };

    let expanded = quote! {
        impl ::atomiq::Atomizable for #name {
            type Atom = #field_type;
            
            fn pack(self) -> Self::Atom {
                #pack_line
            }
            
            fn unpack(atom: Self::Atom) -> Self {
                #unpack_line
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(BitAtomizable)]
pub fn derive_bit_atomizable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl ::atomiq::BitAtomizable for #name {}
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IntAtomizable)]
pub fn derive_int_atomizable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl ::atomiq::IntAtomizable for #name {}
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_atomiq_derive() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass_*.rs");
        t.compile_fail("tests/fail_*.rs");
    }
}