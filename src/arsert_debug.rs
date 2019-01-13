/// Ensure that an expression evaluates to `true` at runtime.
///
/// Much like [`debug_assert!`][debug_assert], `debug_arsert!` invokes the
/// `panic!` macro if the assertion fails, but it also provides
/// information about the values leading to the failed assertion.
///
/// Also like [`debug_assert!`][debug_assert], the runtime check
/// introduced by this macro will only be compiled into your code in debug
/// builds (those with the `debug_assertions` feature enabled).
///
/// [debug_assert]: https://doc.rust-lang.org/stable/std/macro.debug_assert.html
#[macro_export]
macro_rules! arsert_debug {
    ($(arg:tt)*) => (if cfg!(debug_assertions) { arsert!($(arg)*); })
}
