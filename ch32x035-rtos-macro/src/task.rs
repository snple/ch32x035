//! `#[task]` 属性宏实现。

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, ItemFn, Meta, parse_macro_input};

/// 实现 `#[task(priority = N, name = "..", stack = N)]`。
pub fn task_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // 校验：无参数、返回 `!`。
    if !f.sig.inputs.is_empty() {
        return syn::Error::new(Span::call_site(), "task functions must take no arguments")
            .to_compile_error()
            .into();
    }
    let returns_bang = matches!(&f.sig.output, syn::ReturnType::Type(_, ty) if matches!(&**ty, syn::Type::Never(_)));
    if !returns_bang {
        return syn::Error::new(
            f.sig.output.span(),
            "task functions must return `!` (never return)",
        )
        .to_compile_error()
        .into();
    }

    // 解析可选的 `priority` / `name` / `stack`。
    let mut priority: Option<u8> = None;
    let mut name: Option<String> = None;
    let mut stack: Option<usize> = None;

    if !args.is_empty() {
        let meta = parse_macro_input!(args with syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated);
        for m in meta {
            match m {
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident().map(|i| i.to_string());
                    match ident.as_deref() {
                        Some("priority") => match lit_u8(&nv.value) {
                            Some(v) => priority = Some(v),
                            None => {
                                return syn::Error::new_spanned(
                                    &nv.value,
                                    "priority must be a u8 literal",
                                )
                                .to_compile_error()
                                .into();
                            }
                        },
                        Some("name") => match lit_string(&nv.value) {
                            Some(v) => name = Some(v),
                            None => {
                                return syn::Error::new_spanned(
                                    &nv.value,
                                    "name must be a string literal",
                                )
                                .to_compile_error()
                                .into();
                            }
                        },
                        Some("stack") => match lit_usize(&nv.value) {
                            Some(v) => stack = Some(v),
                            None => {
                                return syn::Error::new_spanned(
                                    &nv.value,
                                    "stack must be a usize literal",
                                )
                                .to_compile_error()
                                .into();
                            }
                        },
                        _ => {
                            return syn::Error::new_spanned(
                                &nv,
                                "unknown task argument; expected `priority`, `name` or `stack`",
                            )
                            .to_compile_error()
                            .into();
                        }
                    }
                }
                other => {
                    return syn::Error::new_spanned(
                        other,
                        "task arguments must be `key = value` pairs",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
    }

    let prio = priority.unwrap_or(0);
    if prio > crate::MAX_PRIO_ATTR {
        return syn::Error::new(
            Span::call_site(),
            format!("priority must be <= {}", crate::MAX_PRIO_ATTR),
        )
        .to_compile_error()
        .into();
    }

    let fn_ident = f.sig.ident.clone();
    let task_ident = syn::Ident::new(&format!("__rtos_task_{}", fn_ident), fn_ident.span());
    let stack_ident = syn::Ident::new(&format!("__rtos_stack_{}", fn_ident), fn_ident.span());
    let def_ident = syn::Ident::new(&format!("__rtos_def_{}", fn_ident), fn_ident.span());

    let task_name = name.unwrap_or_else(|| fn_ident.to_string());
    let stack_size = stack.unwrap_or(crate::DEFAULT_STACK_ATTR);
    let attrs: Vec<&Attribute> = f.attrs.iter().collect();
    let vis = &f.vis;
    let stmts = &f.block.stmts;

    quote! {
        #(#attrs)*
        #vis fn #fn_ident() -> ! {
            #(#stmts)*
        }

        // 任务栈：`.bss` 中的普通静态数组。`#[used]` 使其存活。初始栈指针在任务
        // 建立时按 16 字节对齐，数组本身无需特殊对齐。
        #[allow(non_upper_case_globals)]
        #[used]
        static mut #stack_ident: [u8; #stack_size] = [0; #stack_size];

        #[allow(non_upper_case_globals)]
        #[used]
        #[unsafe(link_section = ".rtos.tasks")]
        static #def_ident: ::ch32x035_rtos::task_mod::TaskDef = ::ch32x035_rtos::task_mod::TaskDef {
            entry: #task_ident,
            stack: unsafe { &raw mut #stack_ident } as *mut u8,
            stack_size: #stack_size,
            priority: #prio,
            name: #task_name,
        };

        // 入口即用户函数体（已校验返回 `!`）。
        #[allow(dead_code)]
        unsafe extern "C" fn #task_ident() -> ! {
            #fn_ident()
        }
    }
    .into()
}

fn lit_u8(e: &Expr) -> Option<u8> {
    match e {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(i) => i.base10_parse::<u8>().ok(),
            _ => None,
        },
        _ => None,
    }
}

fn lit_usize(e: &Expr) -> Option<usize> {
    match e {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(i) => i.base10_parse::<usize>().ok(),
            _ => None,
        },
        _ => None,
    }
}

fn lit_string(e: &Expr) -> Option<String> {
    match e {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(s) => Some(s.value()),
            _ => None,
        },
        _ => None,
    }
}
