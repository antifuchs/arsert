use crate::ExpressionInfo;
use crate::TestExpressionInfo;

use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

pub struct BinaryAssertionFailure<'a, L: Debug, R: Debug> {
    left_expr: String,
    right_expr: String,
    op: String,
    left_val: &'a L,
    right_val: &'a R,
}

impl<'a, L: Debug, R: Debug> BinaryAssertionFailure<'a, L, R> {
    pub fn new(
        left_expr: String,
        op: String,
        right_expr: String,
        left_val: &'a L,
        right_val: &'a R,
    ) -> Self {
        BinaryAssertionFailure {
            left_expr,
            op,
            right_expr,
            left_val,
            right_val,
        }
    }
}

impl<'a, L: 'a + Debug, R: 'a + Debug> Display for BinaryAssertionFailure<'a, L, R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} {} {}\n{} = {:?}\n{} = {:?}",
            self.left_expr,
            self.op,
            self.right_expr,
            self.left_expr,
            self.left_val,
            self.right_expr,
            self.right_val
        )
    }
}

impl<'a, L: 'a + Debug, R: 'a + Debug> ExpressionInfo for BinaryAssertionFailure<'a, L, R> {
    fn expression(&self) -> String {
        format!("{} {} {}", self.left_expr, self.op, self.right_expr)
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![
            self.left_expr.to_string(),
            self.op.to_string(),
            self.right_expr.to_string(),
        ]
    }
}

impl<'a, L: 'a + Debug + PartialEq, R: 'a + Debug + PartialEq> TestExpressionInfo<(&L, &R)>
    for BinaryAssertionFailure<'a, L, R>
{
    fn values_equal(&self, val: (&L, &R)) -> Result<(), String> {
        if (self.left_val, self.right_val) == val {
            Ok(())
        } else {
            Err(format!("{:?}", (self.left_val, self.right_val),))
        }
    }
}

pub struct SimpleAssertionFailure {
    expr: String,
    val: bool,
}

impl SimpleAssertionFailure {
    pub fn new(expr: String, val: bool) -> Self {
        SimpleAssertionFailure { val, expr }
    }
}

impl Display for SimpleAssertionFailure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "assertion failed: {:?}", self.val)
    }
}

impl ExpressionInfo for SimpleAssertionFailure {
    fn expression(&self) -> String {
        self.expr.to_string()
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![self.expr.to_string()]
    }
}

impl TestExpressionInfo<bool> for SimpleAssertionFailure {
    fn values_equal(&self, val: bool) -> Result<(), String> {
        if self.val == val {
            Ok(())
        } else {
            Err(format!("{:?}", self.val))
        }
    }
}

pub struct UnaryAssertionFailure<V: Debug> {
    expr: String,
    op: String,
    val: V,
}

impl<V: Debug> UnaryAssertionFailure<V> {
    pub fn new(expr: String, op: String, val: V) -> Self {
        UnaryAssertionFailure { expr, op, val }
    }
}

impl<V: Debug> Display for UnaryAssertionFailure<V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}\n{} = {:?}",
            self.op, self.expr, self.expr, self.val,
        )
    }
}

impl<'a, V: 'a + Debug> ExpressionInfo for UnaryAssertionFailure<V> {
    fn expression(&self) -> String {
        format!("{}{}", self.op, self.expr)
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![self.op.to_string(), self.expr.to_string()]
    }
}

impl<V: Debug + PartialEq> TestExpressionInfo<V> for UnaryAssertionFailure<V> {
    fn values_equal(&self, val: V) -> Result<(), String> {
        if self.val == val {
            Ok(())
        } else {
            Err(format!("{:?}", self.val))
        }
    }
}
