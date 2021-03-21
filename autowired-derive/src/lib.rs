extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
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

    impl_component(&syn::parse(input).unwrap())
}


#[proc_macro_derive(Bean)]
pub fn bean_derive(input: TokenStream) -> TokenStream {
    fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            inventory::submit! {
                autowired::Bean::new_unchecked::<#name>()
            }
        };
        gen.into()
    }

    impl_component(&syn::parse(input).unwrap())
}

