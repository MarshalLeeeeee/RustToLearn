use base_test;

fn test_lib() {
    let x = 2;
    let y = 3;
    println!("x + y = {}", base_test::add(x, y));
}

fn main() {
    test_lib();
}