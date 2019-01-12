use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprPath, Result};

mod binary;
pub(crate) use self::binary::*;

#[derive(Debug)]
pub(crate) enum Assertion {
    Binary(BinaryAssertion),
    Unclear(Expr),
}

use self::Assertion::*;

impl Parse for Assertion {
    fn parse(input: ParseStream) -> Result<Self> {
        let assertion = Expr::parse(input)?;
        Ok(match assertion {
            Expr::Binary(e) => Binary(BinaryAssertion::new(e)),
            a => Unclear(a),
        })
    }
}

impl Assertion {
    pub(crate) fn into_expression(self, panic_fun: ExprPath) -> TokenStream {
        TokenStream::from(match self {
            Binary(b) => b.into_expression(panic_fun),
            Unclear(_expr) => unreachable!(),
        })
    }
}
