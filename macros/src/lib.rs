extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Ident};

#[proc_macro_derive(Hash32)]
pub fn derive_hash32(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let const_ = Ident::from(&*format!("__IMPL_HASH32_FOR_{}", name));
    let hash = compute_hash(&input.data);
    quote!(
        #[allow(non_upper_case_globals)]
        const #const_: () = {
            extern crate hash32;

            impl #impl_generics hash32::Hash for #name #ty_generics #where_clause {
                fn hash<H: hash32::Hasher>(&self, _h: &mut H) -> () {
                    #hash
                }
             }
        };
    ).into()
}

// Add a bound `T: Hash` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(hash32::Hash));
        }
    }
    generics
}

fn compute_hash(data: &Data) -> Tokens {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let fnames = fields.named.iter().map(|f| f.ident);
                quote! {
                    #(
                        hash32::Hash::hash(&self.#fnames, _h);
                    )*
                }
            }
            Fields::Unnamed(ref fields) => {
                let indices = 0..fields.unnamed.len();
                quote! {
                    #(
                        hash32::Hash::hash(&self.#indices, _h);
                    )*
                }
            }
            Fields::Unit => quote!{},
        },
        Data::Enum(..) | Data::Union(..) => {
            panic!("#[derive(Hash)] doesn't currently support `enum` and `union`")
        }
    }
}
