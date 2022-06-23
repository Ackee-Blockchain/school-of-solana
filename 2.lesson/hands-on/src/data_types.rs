use std::mem;

#[allow(dead_code)]
pub fn print_data_types() {
    let string_type = "Solana"; // string type
    let float_type = 4.5; // float type
    let boolean_type = true; // boolean type
    let char_type = '♥'; // unicode character type

    println!("Summer School of {}", string_type);
    println!("Last lesson rating on 5 is{}", float_type);
    println!("We like Solana {}", boolean_type);
    println!("Solana is in our {}", char_type);

    let result = 10; // i32 by default
    println!("Size of i32:{}", mem::size_of::<i32>());

    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;

    println!("Result value is {}", result);
    println!("Sum is {} and age is {}", sum, age);
    println!("Mark is {} and count is {}", mark, count);
}
