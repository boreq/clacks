extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn application_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let func_name = &input.sig.ident;
    let block = &input.block;
    let visibility = &input.vis;
    let sig = &input.sig;

    let expanded = quote! {
        #visibility #sig {
            let type_name_full = std::any::type_name::<Self>();
            let type_name = type_name_full
                .split('<')
                .next()
                .unwrap_or(type_name_full)
                .rsplit("::")
                .next()
                .unwrap_or(type_name_full);
            let start = std::time::Instant::now();
            let result = (|| #block)();
            let duration = start.elapsed();
            match &result {
                Err(error) => {
                    self.metrics.record_application_handler_call(type_name, crate::app::ApplicationHandlerCallResult::Error, crate::domain::time::Duration::new_from_std(duration));
                    log::error!("Application layer call failed: function=`{}` duration=`{:.2?}` error=`{}`", stringify!(#func_name), duration, error.to_string());
                },
                Ok(_) => {
                    self.metrics.record_application_handler_call(type_name, crate::app::ApplicationHandlerCallResult::Ok, crate::domain::time::Duration::new_from_std(duration));
                    log::debug!("Application layer call succeeded: function=`{}` duration=`{:.2?}`", stringify!(#func_name), duration);
                }
            }
            result
        }
    };

    TokenStream::from(expanded)
}
