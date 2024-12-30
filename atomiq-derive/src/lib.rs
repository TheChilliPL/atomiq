extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Atomizable)]
pub fn atomizable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // We get the only field of the struct.
    let field = if let syn::Data::Struct(ref data) = input.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            if fields.named.len() != 1 {
                return TokenStream::from(quote::quote! {
                    compile_error!("Atomizable can only be derived for structs with a single field.");
                });
            }
            fields.named.get(0).unwrap()
        } else if let syn::Fields::Unnamed(ref fields) = data.fields {
            if fields.unnamed.len() != 1 {
                return TokenStream::from(quote::quote! {
                    compile_error!("Atomizable can only be derived for structs with a single field.");
                });
            }
            fields.unnamed.get(0).unwrap()
        } else {
            return TokenStream::from(quote::quote! {
                compile_error!("Atomizable can only be derived for structs with a single field.");
            });
        }
    } else {
        return TokenStream::from(quote::quote! {
            compile_error!("Atomizable can only be derived for structs.");
        });
    };

    let field_name = field
        .ident
        .as_ref();
    
    let field_type = &field.ty;
    
    let pack_line = if let Some(ref field_name) = field_name {
        quote::quote! { self.#field_name }
    } else {
        quote::quote! { self.0 }
    };
    
    let unpack_line = if let Some(ref field_name) = field_name {
        quote::quote! { #name { #field_name: atom } }
    } else {
        quote::quote! { #name(atom) }
    };

    let expanded = quote::quote! {
        impl atomiq::Atomizable for #name {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_atomiq_derive() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass-*.rs");
        t.compile_fail("tests/fail-*.rs");
    }
}