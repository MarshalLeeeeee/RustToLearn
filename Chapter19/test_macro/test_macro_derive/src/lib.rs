use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyTrait2)]
pub fn my_trait2_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast : syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait2 for #name {
            fn mt2_f(&self) {
                println!("{}::mt2_f", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn attr_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(input as syn::ItemFn);

    let name = &func.sig.ident;
    let block = &func.block;
    let vis = &func.vis;
    let sig = &func.sig;
    let generics = &func.sig.generics;
    let gen = quote! {
        #vis #sig #generics {
            println!("Function {} starts.......", stringify!(#name));
            #block
            println!("Function {} ends.......", stringify!(#name));
        }
    };

    gen.into()
}

#[proc_macro]
pub fn func_macro(input: TokenStream) -> TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);
    let gen = quote! {
        {
            println!("Start executing {}", stringify!(#expr));
            let obj = #expr;
            println!("Stop executing {}", stringify!(#expr));
            obj
        }
    };
    gen.into()
}
