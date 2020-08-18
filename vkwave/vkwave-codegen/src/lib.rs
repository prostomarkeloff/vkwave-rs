use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, FnArg, Lit, NestedMeta};

fn parse_filters(args: syn::AttributeArgs) -> Vec<proc_macro2::TokenStream> {
    let mut filters = vec![];
    args.iter()
        .filter_map(|x| match x {
            NestedMeta::Lit(l) => Some(l),
            _ => None,
        })
        .for_each(|x| match x {
            Lit::Str(s) => {
                let b = syn::parse_str::<syn::Expr>(s.value().as_str()).unwrap();
                filters.push(quote! {#b})
            }
            _ => {}
        });
    filters
}

fn parse_args(sig: &syn::Signature) -> Vec<syn::Type> {
    let mut args = vec![];
    sig.inputs.iter().for_each(|x| match x {
        FnArg::Typed(t) => {
            args.push(*t.ty.clone());
        }
        _ => unreachable!(),
    });
    args
}

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);

    let filters = parse_filters(args.clone());

    let ast: syn::ItemFn = syn::parse(input.clone()).unwrap();

    let sig = &ast.sig;
    let indent = sig.ident.clone();
    let args = parse_args(sig);
    let return_type = {
        match sig.output.clone() {
            syn::ReturnType::Default => quote! {()},
            syn::ReturnType::Type(_, t) => {
                quote! {#t}
            }
        }
    };

    let stream = quote! {
        #[allow(non_camel_case_types)]
        pub struct #indent;

        impl #indent {
            // return true if handler was executed and false if not
            async fn handle(&self, ev: vkwave::bots::event::RawContext) -> bool {
                use vkwave::bots::event::FromRawContext;
                #ast
                #indent(#(
                    match <#args>::from_raw_context(ev.clone()) {
                        Some(s) => s,
                        None => return false
                    }
                ),*).await;
                //#indent(ev).await;
                true
            }
        }

    };
    stream.into()
}
