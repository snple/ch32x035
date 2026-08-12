//! Implementation helpers for `#[bind(IRQ)]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, ItemFn, Token, parse_macro_input, punctuated::Punctuated};

/// Valid interrupt names for `#[bind(IRQ)]`.
///
/// Mirrors the `Interrupt` enum generated in `ch32x035-pac/src/generated/mod.rs`.
/// Keep in sync when the SVD or the PAC generator changes.
const VALID_IRQS: &[&str] = &[
    "WWDG_IRQ",
    "PVD_IRQ",
    "FLASH_IRQ",
    "EXTI7_0_IRQ",
    "AWU_IRQ",
    "DMA1_CHANNEL1_IRQ",
    "DMA1_CHANNEL2_IRQ",
    "DMA1_CHANNEL3_IRQ",
    "DMA1_CHANNEL4_IRQ",
    "DMA1_CHANNEL5_IRQ",
    "DMA1_CHANNEL6_IRQ",
    "DMA1_CHANNEL7_IRQ",
    "ADC1_IRQ",
    "I2C1_EV_IRQ",
    "I2C1_ER_IRQ",
    "USART1_IRQ",
    "SPI1_IRQ",
    "TIM1_BRK_IRQ",
    "TIM1_UP_IRQ",
    "TIM1_TRG_COM_IRQ",
    "TIM1_CC_IRQ",
    "TIM2_UP_IRQ",
    "USART2_IRQ",
    "EXTI15_8_IRQ",
    "EXTI25_16_IRQ",
    "USART3_IRQ",
    "USART4_IRQ",
    "DMA1_CHANNEL8_IRQ",
    "USBFS_IRQ",
    "USBFS_WKUP_IRQ",
    "PIOC_IRQ",
    "OPA_IRQ",
    "USBPD_IRQ",
    "USBPD_WKUP_IRQ",
    "TIM2_CC_IRQ",
    "TIM2_TRG_COM_IRQ",
    "TIM2_BRK_IRQ",
    "TIM3_IRQ",
];

/// Implementation of the `#[bind(IRQ)]` attribute macro.
pub fn bind_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // Parse arguments — only the interrupt name is accepted
    let args_parsed = parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_terminated);

    if args_parsed.is_empty() {
        return syn::Error::new(Span::call_site(), "Expected #[bind(IRQ_NAME)]")
            .to_compile_error()
            .into();
    }

    if args_parsed.len() > 1 {
        return syn::Error::new_spanned(
            &args_parsed[1],
            "#[bind] takes only the interrupt name — set priority via interrupt::set_priority() before interrupt::enable()",
        )
        .to_compile_error()
        .into();
    }

    // Get interrupt name (first argument)
    let irq_name = match &args_parsed[0] {
        Expr::Path(path) => {
            let seg = path.path.segments.last().unwrap();
            seg.ident.clone()
        }
        _ => {
            return syn::Error::new_spanned(&args_parsed[0], "Expected interrupt name")
                .to_compile_error()
                .into();
        }
    };

    let user_fn_name = &f.sig.ident;
    let fn_body = &f.block;
    let fn_vis = &f.vis;
    let fn_attrs = &f.attrs;

    let irq_name_str = irq_name.to_string();

    // Validate the interrupt name against the known CH32X035 interrupt set.
    // A misspelled name would otherwise silently link to `DefaultHandler`
    // via the `PROVIDE` fallback in the linker script.
    if !VALID_IRQS.contains(&irq_name_str.as_str()) {
        let valid = VALID_IRQS.join(", ");
        return syn::Error::new_spanned(
            &args_parsed[0],
            format!("unknown interrupt `{irq_name_str}`; valid interrupts are: {valid}"),
        )
        .to_compile_error()
        .into();
    }

    // Generate the interrupt handler: export the user function with the
    // interrupt's symbol name (overriding the weak `device.x` provide).
    let expanded = quote! {
        #(#fn_attrs)*
        #[allow(non_snake_case)]
        #fn_vis fn #irq_name() {
            #fn_body
        }

        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[unsafe(export_name = #irq_name_str)]
        pub unsafe extern "C" fn #user_fn_name() {
            #irq_name()
        }
    };

    expanded.into()
}
