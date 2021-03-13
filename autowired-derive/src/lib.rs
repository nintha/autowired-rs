extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Autowired)]
pub fn autowired_derive(input: TokenStream) -> TokenStream {
    impl_autowired(&syn::parse(input).unwrap())
}

fn impl_autowired(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn new_instance() -> Pin<Box<dyn Future<Output=Result<Arc<Self>, Box<dyn Error>>>>> {
                Box::pin(async {
                    Ok(Arc::new(Default::default()))
                })
            }
        }
    };
    gen.into()
}