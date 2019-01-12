use crate::ExpressionInfo;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

pub struct BinaryAssertionFailure<L: Debug, R: Debug> {
    left_expr: String,
    right_expr: String,
    op: String,
    left_val: L,
    right_val: R,
}

impl<L: Debug, R: Debug> BinaryAssertionFailure<L, R> {
    pub fn new(
        left_expr: String,
        op: String,
        right_expr: String,
        left_val: L,
        right_val: R,
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

impl<L: Debug, R: Debug> Display for BinaryAssertionFailure<L, R> {
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

impl<L: 'static + Debug, R: 'static + Debug> ExpressionInfo for BinaryAssertionFailure<L, R> {
    fn expression(&self) -> String {
        format!("{} {} {}", self.left_expr, self.op, self.right_expr)
    }

    fn values(self) -> HashMap<String, Box<dyn Debug>> {
        let mut ret: HashMap<String, Box<dyn Debug>> = HashMap::new();
        (&mut ret).insert(self.left_expr.to_string(), Box::new(self.left_val));
        (&mut ret).insert(self.right_expr.to_string(), Box::new(self.right_val));
        ret
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![
            self.left_expr.to_string(),
            self.op.to_string(),
            self.right_expr.to_string(),
        ]
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

    fn values(self) -> HashMap<String, Box<dyn Debug>> {
        let mut ret: HashMap<String, Box<dyn Debug>> = HashMap::new();
        (&mut ret).insert(self.expr.to_string(), Box::new(self.val));
        ret
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![self.expr.to_string()]
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

impl<V: 'static + Debug> ExpressionInfo for UnaryAssertionFailure<V> {
    fn expression(&self) -> String {
        format!("{}{}", self.op, self.expr)
    }

    fn values(self) -> HashMap<String, Box<dyn Debug>> {
        let mut ret: HashMap<String, Box<dyn Debug>> = HashMap::new();
        (&mut ret).insert(self.expr.to_string(), Box::new(self.val));
        ret
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![self.op.to_string(), self.expr.to_string()]
    }
}
