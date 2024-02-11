// fn test_for_reference() {
//     let s = String::from("hello world");
//     for c in s.chars() {
//         print!("{}", c);
//     }
//     println!("");
//     let s = s.as_bytes();
//     for c in s.iter() {
//         print!("{}", c);
//     }
//     println!("");
//     for &c in s.iter() {
//         print!("{}", c);
//     }
//     println!("");
// }

// &str type can accept both &str and &String
fn do_string_slice(s: &str) -> &str {
    for (_i, &_b) in s.as_bytes().iter().enumerate() {
        if _b == b' ' {
            return &s[.._i];
        }
    }
    &s[..]
}

fn test_for_string_slice() {
    let s = String::from("hello world");
    let first_word = &s[..5];
    let second_word = &s[6..];
    println!("first word is {} \nsecond word is {}", first_word, second_word);
    println!("first word is {}", do_string_slice(&s[..9]));
    println!("first word is {}", do_string_slice(&String::from("hello_world")));
    let ss: &str = &String::from("hello world"); // reference of String can deref to &str
    println!("ss is {}", ss);
}

fn do_array_slice(arr: &[i32]) -> &[i32] {
    &arr[3..]
}

fn test_for_array_slice() {
    let arr = [1, 2, 3, 4, 5];
    let arr_tail = &arr[3..];
    println!("arr_tail is {:?}", arr_tail);
    println!("arr_tail is {:?}", do_array_slice(&arr));
}

fn main() {
    // test_for_reference();
    test_for_string_slice();
    test_for_array_slice();
}
