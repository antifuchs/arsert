use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

/// A trait representing information about the expression that led to
/// a failed assertion.
pub trait ExpressionInfo: Display {
    /// Returns the entire expression, represented as Rust source code
    /// in a String.
    fn expression(&self) -> String;

    /// Returns the actual values that contributed to the assertion failure.
    fn values(self) -> HashMap<String, Box<dyn Debug>>;

    /// Returns the parts of the expression, broken into rust source
    /// code in a String.
    fn expression_parts(&self) -> Vec<String>;
}

/// Panics with the given assertion information.
pub fn panic_on_failed_assertion(expr: impl ExpressionInfo) {
    panic!(format!("Assertion failed: {}", expr));
}

pub mod expressions;
pub use self::expressions::*;
