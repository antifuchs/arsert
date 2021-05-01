# This project is quasi-abandoned

While I thought this could be a fun exercise to learn rust proc macros, the ecosystem has both moved on quickly enough (allowing tests that return `Result`) and stayed in place long enough (not stabilizing expression proc macros) that this crate is now useless (or, worse than that, actively harmful). Don't use this.

[![Build Status](https://circleci.com/gh/antifuchs/arsert/tree/master.svg?style=shield)](https://circleci.com/gh/antifuchs/arsert/tree/master)

# arsert

## arsert - assertions that fail very sophisticatedly

This crate allows you to write assertions like you would with a
builtin [`assert`][assert], but when the assertion fails, it outputs
diagnostic information about the parameters to the assertion.

### Usage

Here's a failing assertion:

```rust
use arsert::arsert;
let x = 1;
let y: i32 = 2;
arsert!(x >= y.pow(3));
```

This outputs:

```
thread 'main' panicked at 'x >= y . pow ( 3 )
x = 1
y . pow ( 3 ) = 8', arsert_failure/src/lib.rs:23:5
```

Here's a successful one:

```rust
use arsert::arsert;
let x = 20 as i64;
arsert!(x <= x.pow(3));
```

### Supported operations

Right now, arsert supports "simple" assertions (very much like
assert does), unary assertions (e.g. `*foo` and `!foo`), and
assertions on binary operations, like `==`, `>`, `&&` and so on.

I'm working on more supported expressions (and maybe, once proc_macros
as statements get stabilized, an extension mechanism).

### The Name

Sorry for the toilet humor (everybody poops, y'all). Name improvement
suggestions gladly accepted, provided the resulting name is terse and
meaningful.

License: MIT
