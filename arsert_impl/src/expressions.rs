use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprPath, Result};

mod binary;
mod simple;
mod unary;
pub(crate) use self::binary::*;
pub(crate) use self::simple::*;
pub(crate) use self::unary::*;

#[derive(Debug)]
pub(crate) enum Assertion {
    Binary(BinaryAssertion),
    Unary(UnaryAssertion),
    Unclear(SimpleAssertion),
}

use self::Assertion::*;

impl Parse for Assertion {
    fn parse(input: ParseStream) -> Result<Self> {
        let assertion = Expr::parse(input)?;
        Ok(match assertion {
            Expr::Binary(e) => Binary(BinaryAssertion::new(e)),
            Expr::Unary(e) => Unary(UnaryAssertion::new(e)),
            a => Unclear(SimpleAssertion::new(a)),
        })
    }
}

impl Assertion {
    pub(crate) fn into_expression(self, panic_fun: ExprPath) -> TokenStream {
        TokenStream::from(match self {
            Binary(b) => b.into_expression(panic_fun),
            Unary(e) => e.into_expression(panic_fun),
            Unclear(e) => e.into_expression(panic_fun),
        })
    }
}
