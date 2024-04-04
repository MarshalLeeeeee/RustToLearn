use std::result::Result;
use std::io::{self, Error};

type MY_I = i32;
type MY_RESULT<T> = Result<T, Error>;

fn get_input() -> MY_RESULT<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn test_type_alias() {
    let x: i32 = 5;
    let y: MY_I = 6;
    println!("x: {}, y: {}, x+y: {}", x, y, x+y);

    match get_input() {
        Ok(s) => { println!("Get buffer {}", s); },
        Err(_) => { println!("No buffer"); },
    }
}

fn test_dynamic_sized_type() {
    // str is a dynamic sized type
    // dny trait is a dynamic sized type

    // DSTs are usually wrapped in reference or pointer
    // let s: str = "Hello";
    let s: &str = "Hello";
    let s: *const str = "Hello";
}

// fn never_func() -> ! {} // Error, return nothing actually returns (), not !
// panic and infinite loop return !
fn never_func() -> ! {
    panic!("Panic!");
}

// never type is or with the other type
// as long as there are no two non-never types, the return type is inferred by the compiler
fn type_mix_with_never(b: bool) -> i32 {
    if b { 1 }
    else { never_func() }
}

fn test_never_type() {
    // let _ = never_func(); // Error: return nothing should not be caught by any variable, even _
    let x = type_mix_with_never(true); println!("x : {}", x);
    let x = type_mix_with_never(false); println!("x : {}", x);
}

fn main() {
    test_type_alias();
    test_dynamic_sized_type();
    test_never_type();
}
