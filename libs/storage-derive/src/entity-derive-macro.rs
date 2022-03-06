use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Entity)]
pub fn entity_macro(input: TokenStream) -> TokenStream {
  // Parse the string representation
  let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);

  let table_name: String = ident.to_string().to_snake_case();
  (quote! {
      impl ::storage::Entity for #ident {
          fn get_table_name() -> &'static str {
              #table_name
          }
      }
  })
  .into()
}
