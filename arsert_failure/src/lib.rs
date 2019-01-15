#![deny(warnings)]

use std::fmt::Debug;
use std::fmt::Display;

/// A trait that test code can use to check that an assertion failure
/// is represented correctly.
pub trait ExpressionInfo: Display {
    /// Returns the entire expression, represented as Rust source code
    /// in a String.
    fn expression(&self) -> String;

    /// Returns the parts of the expression, broken into rust source
    /// code in a String.
    fn expression_parts(&self) -> Vec<String>;
}

/// A trait that can be used only in test code to check an assertion failure.
pub trait TestExpressionInfo<T: PartialEq + Debug> {
    /// Tests whether an expression failed with the given values. If
    /// they differ, returns the debug representation of the value as
    /// an Err.
    fn values_equal(&self, val: T) -> Result<(), String>;
}

/// Panics with the given assertion information.
#[allow(clippy::needless_pass_by_value)] // necessary so we can use the trait in tests
pub fn panic_on_failed_assertion(expr: impl ExpressionInfo) {
    panic!(format!("{}", expr));
}

pub mod expressions;
pub use self::expressions::*;
