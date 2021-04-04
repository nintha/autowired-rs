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

#[proc_macro_attribute]
pub fn bean(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let is_lazy = args.get(0).map(|x| {
        if let NestedMeta::Meta(nv) = x {
            return nv.path().is_ident("lazy");
        } else {
            return false;
        }
    }).unwrap_or_default();

    let func = syn::parse_macro_input!(input as ItemFn);

    let block = &func.block;
    let vis = &func.vis;
    let name = &func.sig.ident;
    let output = &func.sig.output;

    let submit_method = if is_lazy {
        quote! { autowired::Bean::from_fn_lazy(#name) }
    } else {
        quote! { autowired::Bean::from_fn(#name) }
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

