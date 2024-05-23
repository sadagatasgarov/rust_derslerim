extern crate proc_macro;

use chrono::prelude::*;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

use darling::FromMeta;
use syn::{parse_macro_input, parse_quote, AttributeArgs, ItemFn};

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    input.block.stmts.insert(
        0,
        parse_quote! {
            println!("[Info] calling {}", stringify!(#fn_name));
        },
    );

    input.to_token_stream().into()
}

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {} {}", stringify!(#name), msg);
            }

            fn warn(&self, msg: &str) {
                println!("[Warn] {} {}", stringify!(#name), msg);
            }

            fn error(&self, msg: &str) {
                println!("[Error] {} {}", stringify!(#name), msg);
            }
        }
    };
    trait_impl.into()
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!(" {}", time));
            }
            _ => {
                output.push_str(&format!(" {}", token_string));
            }
        }
    }

    TokenStream::from(quote! {
        println!("{}", #output);
    })
}
