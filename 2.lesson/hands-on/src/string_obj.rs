#[allow(dead_code)]
pub fn print_string_obj() {
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let content_string = String::from("AckeeBlockchain");
    println!("length is {}", content_string.len());
}
