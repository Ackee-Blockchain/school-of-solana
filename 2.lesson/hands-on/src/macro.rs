// ~vec!, without optimizations and other forms
macro_rules! my_vec_macro {
    ( $( $item:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($item); )*
            temp_vec
        }
    };
}
// fn main() {
//     let mut my_vector = Vec::new();
//     my_vector.push(1);
//     my_vector.push(2);

//     let my_vector_2 = vec![1, 2];
//     let my_vector_3 = my_vec_macro![1, 2];

//     assert!(all_equal(&[&my_vector, &my_vector_2, &my_vector_3,]));
// }

// fn all_equal<T: PartialEq>(items: &[T]) -> bool {
//     items.windows(2).all(|windows| windows[0] == windows[1])
// }
