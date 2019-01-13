//! # arsert - assertions that fail very sophisticatedly
//!
//! This crate allows you to write assertions like you would with the
//! builtin [`assert!`][assert], but when the assertion fails, it outputs
//! diagnostic information about the parameters to the assertion.
//!
//! The output is similar to what [`assert_eq!`][assert_eq] and
//! [`assert_ne!`][assert_ne] do, but it supports more operators out of
//! the box, and often doesn't require the author of a test to re-word
//! failure messages.
//!
//! ## Usage
//!
//! Here's a failing assertion:
//!
//! ```rust,should_panic
//! use arsert::arsert;
//! let x = 1;
//! let y: i32 = 2;
//! arsert!(x >= y.pow(3));
//! ```
//!
//! This outputs:
//!
//! ```text
//! thread 'main' panicked at 'x >= y . pow ( 3 )
//! x = 1
//! y . pow ( 3 ) = 8', arsert_failure/src/lib.rs:23:5
//! ```
//!
//! Here's a successful one:
//!
//! ```rust
//! use arsert::arsert;
//! let x = 20 as i64;
//! arsert!(x <= x.pow(3));
//! ```
//!
//! ## Supported operations
//!
//! Right now, arsert supports "simple" assertions (very much like
//! assert does), unary assertions (e.g. `*foo` and `!foo`), and
//! assertions on binary operations, like `==`, `>`, `&&` and so on.
//!
//! I'm working on more supported expressions (and maybe, once proc_macros
//! as statements get stabilized, an extension mechanism).
//!
//! ## The Name
//!
//! Sorry for the toilet humor (everybody poops, y'all). Name improvement
//! suggestions gladly accepted, provided the resulting name is terse and
//! meaningful.
//!
//! [assert]: https://doc.rust-lang.org/stable/std/macro.assert.html
//! [assert_eq]: https://doc.rust-lang.org/stable/std/macro.assert_eq.html
//! [assert_ne]: https://doc.rust-lang.org/stable/std/macro.assert_ne.html

#![deny(warnings)]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use arsert_impl::arsert;

pub use arsert_failure::{panic_on_failed_assertion, ExpressionInfo};

mod arsert_debug;
pub use self::arsert_debug::*;
