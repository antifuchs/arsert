use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{ExprBinary, ExprPath};

#[derive(Debug)]
pub(crate) struct BinaryAssertion(ExprBinary);

impl BinaryAssertion {
    pub(crate) fn new(b: ExprBinary) -> Self {
        BinaryAssertion(b)
    }

    pub(super) fn into_expression(self, panic_fun: ExprPath) -> TokenStream {
        let left = self.0.left.into_token_stream();
        let left_src = format!("{}", left);
        let op = self.0.op.into_token_stream();
        let op_src = format!("{}", op);
        let right = self.0.right.into_token_stream();
        let right_src = format!("{}", right);

        TokenStream::from(quote! {
            {
                let left = #left;
                let right = #right;
                if !(left #op right) {
                    #panic_fun(::arsert_failure::BinaryAssertionFailure::new(#left_src.to_string(),
                                                                             #op_src.to_string(),
                                                                             #right_src.to_string(),
                                                                             left,
                                                                             right));
                }
            }
        })
    }
}
