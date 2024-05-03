/// Checks if the given string meets the minimum length requirement.
///
/// This function examines the length of a string reference, `s`, and determines if it is
/// at least as long as the specified minimum length. If `s` meets or exceeds
/// the minimum length, the function returns `s` wrapped in an `Ok`. If it does not,
/// it returns an error message indicating the string is not long enough.
pub fn check_length(s: &str) -> Result<&str, String> {
    if s.chars().count() >= 10 {
        return Ok(s);
    } else {
        return Err(format!("'{}' is not long enough!", s));
    }
}

fn main() {
    let result1 = check_length("Hello, world!");
    let result2 = check_length("Hello");
    println!("Result1 is: {:#?}", result1);
    println!("Result2 is: {:#?}", result2);
}
