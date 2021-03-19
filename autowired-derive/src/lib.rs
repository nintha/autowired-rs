extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    impl_component(&syn::parse(input).unwrap())
}

fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            type Error = Box<dyn std::error::Error + Send + Sync>;

            fn new_instance() -> Result<Arc<Self>, Self::Error> {
               Ok(Arc::new(Default::default()))
            }
        }
    };
    gen.into()
}