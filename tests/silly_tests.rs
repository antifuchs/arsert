use arsert::arsert;
use arsert_failure::TestExpressionInfo;
use arsert_failure::{
    BinaryAssertionFailure, ExpressionInfo, SimpleAssertionFailure, UnaryAssertionFailure,
};
use std::ops::Not;

#[derive(Debug, PartialEq)]
struct NonCopy(bool);

impl Not for &NonCopy {
    type Output = bool;

    fn not(self) -> bool {
        !self.0
    }
}

#[test]
fn binary_non_copy() {
    let x = NonCopy(false);
    let y = NonCopy(true);

    let mut ran = false;
    let mut validate = |ei: BinaryAssertionFailure<NonCopy, NonCopy>| {
        assert_eq!(Ok(()), ei.values_equal((&x, &y)));
        ran = true;
    };
    arsert!(#![failure_function(validate)] x == y);
    assert!(ran);
}

#[test]
fn unary_non_copy() {
    let x = NonCopy(true);
    let mut ran = false;
    let mut validate = |ei: UnaryAssertionFailure<&NonCopy>| {
        assert_eq!(Ok(()), ei.values_equal(&x));
        ran = true;
    };
    arsert!(#![failure_function(validate)] !(&x));
    assert!(ran);
}

#[test]
fn binary_ops() {
    let x = 1;
    let y = 2;
    let mut ran = false;
    let mut validate = |ei: BinaryAssertionFailure<i32, i32>| {
        assert_eq!("x >= y", ei.expression());
        assert_eq!(vec!["x", ">=", "y"], ei.expression_parts());

        assert_eq!(Ok(()), ei.values_equal((&1, &2)));
        ran = true;
    };
    arsert!(#![failure_function(validate)] x >= y);
    assert!(ran);
}

#[test]
fn unary_ops() {
    let x = Box::new(false);
    let mut ran = false;
    let mut validate = |ei: UnaryAssertionFailure<Box<bool>>| {
        assert_eq!("*x", ei.expression());
        assert_eq!(vec!["*", "x"], ei.expression_parts());

        assert_eq!(Ok(()), ei.values_equal(Box::new(false)));
        ran = true;
    };
    arsert!(#![failure_function(validate)] *x);
    assert!(ran);
}

#[test]
fn unclear_ops() {
    let mut ran = false;
    let mut validate = |ei: SimpleAssertionFailure| {
        assert_eq!("something", ei.expression());
        assert_eq!(vec!["something"], ei.expression_parts());

        assert_eq!(Ok(()), ei.values_equal(false));
        ran = true;
    };
    let something = false;
    arsert!(#![failure_function(validate)] something);
    assert!(ran);
}
