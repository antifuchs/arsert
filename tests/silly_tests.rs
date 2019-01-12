use arsert::arsert;
use arsert_failure::{
    BinaryAssertionFailure, ExpressionInfo, SimpleAssertionFailure, UnaryAssertionFailure,
};

#[test]
fn binary_ops() {
    let x = 1;
    let y = 2;
    let validate = |ei: BinaryAssertionFailure<i32, i32>| {
        assert_eq!("x >= y", ei.expression());
        assert_eq!(vec!["x", ">=", "y"], ei.expression_parts());

        let vals = ei.values();
        assert_eq!(
            Some("1".to_string()),
            vals.get("x").map(|v| format!("{:?}", v))
        );
        assert_eq!(
            Some("2".to_string()),
            vals.get("y").map(|v| format!("{:?}", v))
        );
    };
    arsert!(#![failure_function validate] x >= y);
}

#[test]
fn unary_ops() {
    let x = true;
    let validate = |ei: UnaryAssertionFailure<bool>| {
        assert_eq!("!x", ei.expression());
        assert_eq!(vec!["!", "x"], ei.expression_parts());

        let vals = ei.values();
        assert_eq!(
            Some("true".to_string()),
            vals.get("x").map(|v| format!("{:?}", v))
        );
    };
    arsert!(#![failure_function validate] !x);
}

#[test]
fn unclear_ops() {
    let validate = |ei: SimpleAssertionFailure| {
        assert_eq!("something", ei.expression());
        assert_eq!(vec!["something"], ei.expression_parts());

        let vals = ei.values();
        assert_eq!(
            Some("false".to_string()),
            vals.get("something").map(|v| format!("{:?}", v))
        );
    };
    let something = false;
    arsert!(#![failure_function validate] something);
}
