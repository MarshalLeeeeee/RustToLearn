fn test_vector_index_overflow() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _x = v[100]; // call panic! implicitly
}

fn test_explicit_panic() {
    panic!("test panic");
}

fn main() {
    test_vector_index_overflow();
    test_explicit_panic();
}
