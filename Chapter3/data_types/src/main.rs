fn test_integer() {
    let a: i32 = 1_000_000; // use _ to separate number for readability
    println!("The value of a is {a}");
    // let b: u8 = -1; // Error: convert -1 to u8
    // let b: u8 = 256; // Error: convert -1 to u8 [0, 255]
    let b: u8 = 255;
    println!("The value of b is {b}");
    // let c: i32 = 127u64; // Error: mismatch of big type to small type
    // let c: i32 = 127i8; // Error: mismatch of type
    // let c: isize = 127i64; // Error: unsafe match of type for cross architecture
    let c: isize = 127_isize;
    println!("The value of c is {c}");
    let d: i8 = 100;
    // let d = d + 100; // Error: mismatch of domain, fail in cargo build
    // let d = d * 2; // Error: mismatch of domain
    println!("The value of d is {d}");
    let mut e: u8 = d.try_into().unwrap(); // convert explicitly
    // e = e + 500; // Error: mut variable overflow is not detected in compile, panic raise in runtime (debug)
    e = e + 1;
    println!("The value of e is {e}");
}

fn test_float() {
    let f: f32 = 3.14;
    println!("The value of f is {f}");
}

fn test_bool() {
    let g: bool = false;
    println!("The value of g is {g}");
}

fn test_char() {
    let h: char = '啊';
    println!("The value of h is {h}");
}

fn test_tuple() {
    let tup: (i32, f32, bool) = (500, 6.4, true);
    // let (x, y) = tup; // Error: unpack count un match 
    let (x, y, z) = tup;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");
    let xx = tup.0;
    let yy = tup.1;
    let zz = tup.2;
    println!("The value of xx is {xx}");
    println!("The value of yy is {yy}");
    println!("The value of zz is {zz}");
}

fn test_array() {
    // let arr = [500, 6.4, true]; // Error: mixed object types
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr_i0 = arr[0];
    println!("The value of arr_i0 is {arr_i0}");
    let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr_i0 = arr[0];
    println!("The value of arr_i0 is {arr_i0}");
    let arr: [i8; 9] = [9; 9];
    let arr_i0 = arr[0];
    println!("The value of arr_i0 is {arr_i0}");
    // let arr_i10 = arr[10]; // Error: runtime panic whenever index overflow happens
}

fn main() {
    test_integer();
    test_float();
    test_bool();
    test_char();
    test_tuple();
    test_array();
}
