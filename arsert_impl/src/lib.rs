extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{self, parse_macro_input, Attribute, Expr, ExprPath, Result, Token};

mod expressions;

// use self::expressions::*;

#[derive(Debug)]
struct ArsertExpression {
    failure_function: Option<ExprPath>,
    assertion: expressions::Assertion,
    format_expr: Punctuated<Expr, Token![,]>,
}

impl Parse for ArsertExpression {
    fn parse(input: ParseStream) -> Result<Self> {
        let failure_function = if input.peek(Token![#]) {
            let attrs = input.call(Attribute::parse_inner)?;
            if let Some(attr) = attrs
                .into_iter()
                .find(|a| a.path.is_ident("failure_function"))
            {
                let expr: ExprPath = syn::parse2(attr.tts)?;
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let assertion = expressions::Assertion::parse(input)?;
        let format_expr: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(input)?;
        Ok(ArsertExpression {
            assertion,
            format_expr,
            failure_function,
        })
    }
}

impl Into<TokenStream> for ArsertExpression {
    fn into(self) -> TokenStream {
        let default: ExprPath = syn::parse_str("arsert::panic_on_failed_assertion").unwrap();
        self.assertion
            .into_expression(self.failure_function.unwrap_or(default))
    }
}

#[proc_macro_hack]
pub fn arsert(tokens: TokenStream) -> TokenStream {
    let input: ArsertExpression = parse_macro_input!(tokens as ArsertExpression);

    #[cfg(test)] // print debug info only outside LSP (otherwise it messes with lsp-mode)
    println!("expression: {:?}", input);

    input.into()
}
