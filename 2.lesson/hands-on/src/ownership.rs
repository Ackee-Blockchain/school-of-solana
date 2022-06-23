#[allow(dead_code)]
pub fn ownership() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{:?}", v2);
}
