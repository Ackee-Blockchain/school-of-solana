// Unit-style tests of pure logic from the crate.
// These run without a validator or Anchor runtime.

use test_examples::math_function;


#[test]
fn math_function_happy_path() {
    assert_eq!(math_function(2), Some(8));
}

#[test]
fn math_function_boundary_and_overflow() {
    // boundary: exactly 10 -> 0
    assert_eq!(math_function(10), Some(0));
    // overflow case: >10 returns None
    assert_eq!(math_function(11), None);
}
