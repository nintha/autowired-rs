use quote::quote;
use proc_macro::TokenStream;
use syn::{ItemFn, AttributeArgs, NestedMeta};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl autowired::Component for #name {
            fn new_instance() -> Option<Self> {
               Some(Default::default())
            }
        }
        autowired::submit! {
            autowired::Bean::new_unchecked::<#name>()
        }
    };

    gen.into()
}

#[proc_macro_derive(LazyComponent)]
pub fn lazy_component_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl autowired::Component for #name {
            fn new_instance() -> Option<Self> {
               Some(Default::default())
            }
        }
        autowired::submit! {
            autowired::Bean::new_unchecked_lazy::<#name>()
        }
    };
    gen.into()
}

/// Full feature example: `#[bean(lazy, option)]`
/// - lazy: submit bean when it be used
/// - option: maybe constructor will return `None`
#[proc_macro_attribute]
pub fn bean(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);

    let mut is_lazy = false;
    let mut is_option = false;
    for meta in args {
        if let NestedMeta::Meta(nv) = meta {
            if nv.path().is_ident("lazy") {
                is_lazy = true;
            }

            if nv.path().is_ident("option") {
                is_option = true;
            }
        }
    }

    let func = syn::parse_macro_input!(input as ItemFn);

    let block = &func.block;
    let vis = &func.vis;
    let name = &func.sig.ident;
    let output = &func.sig.output;

    let submit_method = if is_option {
        quote! { autowired::Bean::from_fn_return_option(#name, #is_lazy) }
    } else {
        quote! { autowired::Bean::from_fn(#name, #is_lazy) }
    };

    let gen = quote! {
        #vis fn #name() #output {
            #block
        }
        autowired::submit! {
            #submit_method
        }
    };

    gen.into()
}

