use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{Expr, ExprPath};

#[derive(Debug)]
pub(crate) struct SimpleAssertion(Expr);

impl SimpleAssertion {
    pub(crate) fn new(e: Expr) -> Self {
        SimpleAssertion(e)
    }

    pub(super) fn into_expression(self, panic_fun: ExprPath) -> TokenStream {
        let expr = self.0.into_token_stream();
        let expr_src = format!("{}", expr);
        TokenStream::from(quote! {
            {
                let val = #expr;
                if !val {
                    let info = ::arsert_failure::SimpleAssertionFailure::new(#expr_src.to_string(), val);
                    #panic_fun(info);
                }
            }
        })
    }
}
