use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{Expr, ExprUnary};

#[derive(Debug)]
pub(crate) struct UnaryAssertion(ExprUnary);

impl UnaryAssertion {
    pub(crate) fn new(b: ExprUnary) -> Self {
        UnaryAssertion(b)
    }

    pub(super) fn into_expression(self, panic_fun: Expr) -> TokenStream {
        let expr = self.0.expr.into_token_stream();
        let expr_src = format!("{}", expr);
        let op = self.0.op.into_token_stream();
        let op_src = format!("{}", op);

        TokenStream::from(quote! {
            {
                let expr = #expr;
                if !(#op expr) {
                    let info = ::arsert_failure::UnaryAssertionFailure::new(#expr_src.to_string(),
                                                                            #op_src.to_string(),
                                                                            expr);
                    #panic_fun(info);
                }
            }
        })
    }
}
