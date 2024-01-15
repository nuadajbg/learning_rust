pub fn safe_integer_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("b must not be 0"))
    } else {
        Ok(a / b)
    }
}