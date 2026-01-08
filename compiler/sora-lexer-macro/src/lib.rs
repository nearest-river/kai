
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{
  ItemEnum,
  token::Comma,
  punctuated::Punctuated,
};


#[proc_macro_attribute]
pub fn impl_token_debug(_attrs: TokenStream,item: TokenStream)-> TokenStream {
  let item=syn::parse::<ItemEnum>(item).unwrap();
  let ty_name=&item.ident;

  let mut arms=Punctuated::<TokenStream2,Comma>::new();
  for variant in &item.variants {
    let variant_name=&variant.ident;

    if !variant.fields.is_empty() {
      arms.push(quote! {
        #ty_name::#variant_name(variant)=> std::fmt::Debug::fmt(variant,f)
      });
    } else {
      arms.push(quote! {
        #ty_name::#variant_name=> f.write_str(stringify!(#variant_name))
      });
    }
  }

  let tokens=quote! {
    #item

    impl std::fmt::Debug for #ty_name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        match self {
          #arms
        }
      }
    }
  };

  tokens.into()
}












