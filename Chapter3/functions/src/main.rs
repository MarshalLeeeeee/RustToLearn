fn test_func_with_param(i_var: i32, c_var: char) {
    println!("func_with_param: {i_var} {c_var}");
}

fn test_func_no_param() {
    println!("func_no_param");
}

fn func_with_implicit_return(i_var: i32) -> i32 {
    i_var
}

fn test_func_with_implicit_return(i_var: i32) {
    let x: i32 = i_var;
    println!("test before func with implicit return: {x}");
    let x = x + func_with_implicit_return(x);
    println!("test after func with implicit return: {x}");
}

fn func_with_explicit_return(i_var: i32) -> i32 {
    return i_var;
}

fn test_func_with_explicit_return(i_var: i32) {
    let x: i32 = i_var;
    println!("test before func with explicit return: {x}");
    let x = x + func_with_explicit_return(x);
    println!("test after func with explicit return: {x}");
}

fn main() {
    test_func_no_param();
    test_func_with_param(42, 'c');
    test_func_with_implicit_return(9);
    test_func_with_explicit_return(9);
}
