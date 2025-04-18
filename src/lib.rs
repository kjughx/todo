use proc_macro::TokenStream;
use quote::quote;
use syn::{ForeignItemFn, ItemFn};

#[proc_macro_attribute]
pub fn todo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();

    let expanded = match syn::parse::<ForeignItemFn>(input) {
        Ok(f) => {
            let vis = f.vis;
            let sig = f.sig;

            quote! {
                #[allow(unused_variables)]
                #[allow(unreachable_code)]
                #vis #sig {
                    todo!()
                }
            }
        }

        Err(_) => match syn::parse::<ItemFn>(input_clone) {
            Ok(f) => {
                let vis = f.vis;
                let sig = f.sig;
                let body = f.block;

                quote! {
                    #[allow(unused_variables)]
                    #[allow(unreachable_code)]
                    #vis #sig {
                        { #body }
                        todo!()
                    }
                }
            }
            Err(e) => return TokenStream::from(e.into_compile_error()),
        },
    };

    TokenStream::from(expanded)
}
