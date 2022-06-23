#[allow(dead_code)]
pub fn flow_control() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

#[allow(dead_code)]
pub fn ternal_like() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
}
