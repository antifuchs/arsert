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
        (&mut ret).insert(self.left_expr.clone(), Box::new(self.left_val));
        (&mut ret).insert(self.right_expr.clone(), Box::new(self.right_val));
        ret
    }

    fn expression_parts(&self) -> Vec<String> {
        vec![
            self.left_expr.clone(),
            self.op.clone(),
            self.right_expr.clone(),
        ]
    }
}
