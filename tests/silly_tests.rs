use arsert::arsert;
use arsert_failure::TestExpressionInfo;
use arsert_failure::{
    BinaryAssertionFailure, ExpressionInfo, SimpleAssertionFailure, UnaryAssertionFailure,
};

#[derive(Debug)]
struct NonCopy(bool);

impl PartialEq for NonCopy {
    fn eq(&self, other: &NonCopy) -> bool {
        self.0 || other.0
    }
}

#[test]
fn binary_non_copy() {
    let x = NonCopy(false);

    let mut ran = false;
    let mut validate = |ei: BinaryAssertionFailure<NonCopy, NonCopy>| {
        // one of us one of us
        let gooble_gobble = NonCopy(true);
        assert_eq!(Ok(()), ei.values_equal((&gooble_gobble, &gooble_gobble)));
        ran = true;
    };
    arsert!(#![failure_function(validate)] x == x);
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
