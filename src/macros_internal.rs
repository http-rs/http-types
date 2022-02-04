/// Return early with an error if a condition is not satisfied.
///
/// This macro is equivalent to `if !$cond { return Err(From::from($err)); }`.
///
/// Analogously to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
/// rather than panicking.
macro_rules! internal_ensure {
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return $crate::private::Err($err.into());
        }
    };
}

/// Return early with an error if two expressions are not equal to each other.
///
/// This macro is equivalent to `if $left != $right { return Err(From::from($err)); }`.
///
/// Analogously to `assert_eq!`, `ensure_eq!` takes two expressions and exits the function
/// if the expressions are not equal. Unlike `assert_eq!`, `ensure_eq!` returns an `Error`
/// rather than panicking.
#[macro_export]
macro_rules! internal_ensure_eq {
    ($left:expr, $right:expr, $err:expr $(,)?) => {
        if $left != $right {
            return $crate::private::Err($err.into());
        }
    };
}
