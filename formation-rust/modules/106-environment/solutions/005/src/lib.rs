//! # Exercice 005
//! 
//! This is an exercise crate with the objective of
//! learning how to write documentation.
//! This crate only defines a single function:
//! * [`safe_integer_divide`](fn.safe_integer_divide.html).

/// Safely divide integer `a` by integer `b`.
/// This functions checks that the b is not 0 and returns an error
/// if this is so.
///
/// # Examples
/// 
/// ```
/// let result = exercice_005::safe_integer_divide(4, 2)
///     .expect("should have gotten a value");
/// assert_eq!(result, 2);
/// ```
/// 
/// # Errors
/// 
/// In the case where the `b` parameter is 0, the function
/// returns `Err(String::from("b must not be 0"))`:
/// 
/// ```
/// let result = exercice_005::safe_integer_divide(4, 0);
/// assert!(result.is_err());
/// ```
pub fn safe_integer_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("b must not be 0"))
    } else {
        Ok(a / b)
    }
}