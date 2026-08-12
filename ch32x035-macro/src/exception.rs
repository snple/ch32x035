//! Implementation helpers for `#[exception]` attribute
//!
//! Simplified RISC-V version: the CH32X035 (QingKe V4C) does not push a
//! hardware stack frame for exceptions, so exception handlers take no
//! arguments (unlike the Cortex-M `&ExceptionFrame`).

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Valid RISC-V machine exceptions on the CH32X035 (vector entries 2..=15).
const VALID_EXCEPTIONS: &[&str] = &[
    "DefaultHandler",
    "NonMaskableInt",
    "HardFault",
    "EcallMMode",
    "EcallUMode",
    "BreakPoint",
    "SysTick",
    "Software",
];

/// Implementation of the `#[exception]` attribute macro.
pub fn exception_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let ident = f.sig.ident.clone();
    let ident_str = ident.to_string();

    // The handler name must be one of the known core exceptions
    if !VALID_EXCEPTIONS.contains(&ident_str.as_str()) {
        let valid = VALID_EXCEPTIONS.join(", ");
        return syn::Error::new(
            ident.span(),
            format!("unknown exception `{ident_str}`; valid exceptions are: {valid}"),
        )
        .to_compile_error()
        .into();
    }

    // No arguments are allowed (RISC-V has no hardware exception frame)
    if !f.sig.inputs.is_empty() {
        return syn::Error::new(
            Span::call_site(),
            "exception handlers take no arguments on the CH32X035 (QingKe V4C)",
        )
        .to_compile_error()
        .into();
    }

    let attrs = &f.attrs;
    let stmts = &f.block.stmts;

    quote! {
        #[allow(non_snake_case)]
        #(#attrs)*
        #[unsafe(export_name = #ident_str)]
        pub unsafe extern "C" fn #ident() {
            #(#stmts)*
        }
    }
    .into()
}
