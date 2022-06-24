pub fn get_slices() {
    let ints: [i32; 5] = [1, 2, 3, 4, 5];
    let slice1: &[i32] = &ints[0..3];
    let slice2: &[i32] = &ints[2..4];

    println!("slice1: {:?}", slice1);
    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
}