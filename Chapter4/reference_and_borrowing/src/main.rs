fn return_ownership(s: String) -> (String, usize) {
    let s_len = s.len();
    (s, s_len)
}

fn test_return_ownership() {
    let s = String::from("hello");
    let (s, s_len) = return_ownership(s);
    println!("{} has length {}", s, s_len);
}

fn pass_reference(s: &String) -> usize {
    s.len()
}

fn test_pass_reference() {
    let s = String::from("hello");
    let s_len = pass_reference(&s);
    println!("{} has length {}", s, s_len);
}

fn mod_reference(s: & String) {
    // s.push_str(" world"); // Error: the data it refers to cannot be borrowed as mutable
}

fn mod_mut_reference(s: &mut String) {
    s.push_str(" world"); // mut reference is mutable
}

fn test_mod_reference() {
    let s = String::from("hello");
    mod_reference(&s);
    println!("The new s is {}", s);
    let mut ss = String::from("hello");
    mod_mut_reference(&mut ss);
    println!("The new ss is {}", ss);
}

fn test_declare_reference() {
    let mut s = String::from("hello");

    // Immutable reference of mutable ownership can be overlapped
    let sr1 = &s;
    let sr2 = &s;
    // sr1.push_str(" sr1"); // Error: the reference of mutable ownership is immutable
    println!("sr1 {}", sr1);
    println!("sr2 {}", sr2);

    // Mutable reference cannot wrapped inside an immutable reference
    // to make sure the immutable reference always see the immutable reference
    // let sr3 = &s;
    // let sr4 = &mut s;
    // println!("sr4 {} ", sr4);
    // println!("sr3 {}", sr3);
    
    // Immutable reference cannot wrapped inside a mutable reference
    // to make sure the immutable reference always see the immutable reference
    // let sr5 = &mut s;
    // let sr6 = &s;
    // println!("sr6 {}", sr6);
    // println!("sr5 {} ", sr5);

    // Mutable reference must have an isolated usage span
    let sr7 = &mut s;
    sr7.push_str(" sr7");
    println!("sr7 {}", sr7);
    let sr8 = &mut s;
    sr8.push_str(" sr8");
    println!("sr8 {}", sr8);
    println!("s {}", s);

    // As for a increase in readability and maintainability, we can scope mutable references
    {
        let sr9 = &mut s;
        sr9.push_str(" sr9");
        println!("sr9 {}", sr9);
    }
    {
        let sr10 = &mut s;
        sr10.push_str(" sr10");
        println!("sr10 {}", sr10);
    }
    println!("s {}", s);
}

// Error: return a borrowed reference to a dangling value
// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn test_dangling_reference() {
    // dangling_reference();
}

fn main() {
    test_return_ownership();
    test_pass_reference();
    test_mod_reference();
    test_declare_reference();
    test_dangling_reference();
}
