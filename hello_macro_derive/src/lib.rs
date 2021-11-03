extern crate proc_macro; // Compiler’s API, allows to read and manipulate Rust code

use crate::proc_macro::TokenStream;
use quote::quote; // Turns syn data structures back into Rust code
use syn; // Parses Rust code

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
      impl HelloMacro for #name {
          fn hello_macro() {
              println!("A macro returning «{}»", stringify!(#name));
          }
      }
  };

  gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_hello_macro(&ast)
}
