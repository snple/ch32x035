//! Implementation helpers for `#[main]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    ItemFn, ReturnType, Type,
    parse::{self},
    parse_macro_input,
};

/// Generate the expanded output for `#[main]`.
pub fn main_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // No arguments are currently supported.
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "unexpected arguments; `#[main]` takes no arguments",
        )
        .to_compile_error()
        .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    // Check that the function has no arguments
    if !f.sig.inputs.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[main] function must have signature fn() -> !",
        )
        .to_compile_error()
        .into();
    }

    // Check that the return type is `!` (never)
    match &f.sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Never(_) = **ty {
                // OK
            } else {
                return parse::Error::new(
                    Span::call_site(),
                    "#[main] function must have return type `!`",
                )
                .to_compile_error()
                .into();
            }
        }
        ReturnType::Default => {
            return parse::Error::new(
                Span::call_site(),
                "#[main] function must have return type `!`",
            )
            .to_compile_error()
            .into();
        }
    }

    let attrs = &f.attrs;
    let stmts = &f.block.stmts;
    let ident = &f.sig.ident;

    quote!(
        #[unsafe(export_name = "main")]
        #(#attrs)*
        pub unsafe fn #ident() -> ! {
            unsafe { ch32x035::startup::init() };

            // User code
            #(#stmts)*
        }
    )
    .into()
}
