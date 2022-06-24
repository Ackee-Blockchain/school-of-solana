trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("Integer: {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("Float: {}", self)
    }
}

pub fn main () {
    let answer: i32 = 42;
    let pi: f64 = 3.1415;
    
    // Show
    let s1 = answer.show();
    let s2 = pi.show();

    // Print
    println!("s1: {}, s2: {}", s1, s2);
}