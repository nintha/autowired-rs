use quote::quote;
use proc_macro::TokenStream;
use syn::{ItemFn};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl Component for #name {
                fn new_instance() -> Option<Self> {
                   Some(Default::default())
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
            autowired::submit! {
                autowired::Bean::new_unchecked::<#name>()
            }
        };
        gen.into()
    }

    impl_component(&syn::parse(input).unwrap())
}

#[proc_macro_attribute]
pub fn bean(_: TokenStream, input: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(input as ItemFn);

    let block = &func.block;
    let vis = &func.vis;
    let name = &func.sig.ident;
    let output = &func.sig.output;

    let gen = quote!{
        #vis fn #name() #output {
            #block
        }

        autowired::submit! {
            autowired::Bean::from_fn(#name)
        }
    };
    gen.into()
}

