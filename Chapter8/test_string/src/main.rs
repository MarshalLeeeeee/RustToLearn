fn test_string_init() {
    let mut str1 = String::new();
    println!("str 1 {}", str1);
    str1 = "str1_1".to_string();
    println!("str 1 {}", str1);
    str1 = String::from("str1_2");
    println!("str 1 {}", str1);
}

fn test_string_func() {
    // push API
    let mut s = String::from("hello");
    s.push_str(", world"); // append string literal (& can be ignored) to the tail
    s.push('!'); // append character to the tail
    s.push_str(&String::from(" again")[..]); // append slice of String to the tail
    println!("s: {}", s);
    // + operator
    let mut s2 = String::from("hello");
    s2 += ", world";
    // Error: operator + only takes &str as operand
    // s2 += '!';
    s2 += &String::from("! again"); // using deref coercion to convert &String to &String[..] = &str    
    println!("s2: {}", s2);
    // format! which takes the reference and return a owned String
    let s3 = String::from("hello");
    let s4 = String::from("world");
    let s5 = String::from("again");
    let s6 = format!("{}, {}! {}", s3, s4, s5);
    println!("s3 {}", s3);
    println!("s4 {}", s4);
    println!("s5 {}", s5);
    println!("s6 {}", s6);
    let mut s7 = String::from("你好, hello");
    // get the length of String which is the length of the bytes
    println!("s7 length: {}", s7.len());
    // get the capacity
    println!("s7 capacity: {}", s7.capacity());
    // check if contains substring
    println!("s7 contains hello: {}", s7.contains("hello"));
    // replace substring (not in place)
    s7 = s7.replace("l", "L");
    println!("s7 after replace: {}", s7);
    // replace range in place
    s7.replace_range(..8, "!");
    println!("s7 after replace range: {}", s7);
    // clear the String
    s7.clear();
    println!("s7 length: {}", s7.len());
    println!("s7 : {}", s7);
}

fn test_string_representation() {
    // ASCII
    let s1 = "hello";
    // The first byte of s1 is 104
    // println!("First indexed data is {}", &s1[0]);
    println!("The length of s1 is {}", s1.len());
    // Unicode
    let s2 = "你好"; // b'\xe4\xbd\xa0\xe5\xa5\xbd'
    // The byte of s2's first character is \xe4\xbd\xa0
    // println!("First indexed data is {}", &s2[0]);
    println!("The bytes of s2 is {:?}", s2.bytes()); // the length is 6
    println!("The length of s2 is {}", s2.len()); // the length is 6
    // slicing, the slicing unit is byte
    let s3 = "你好, hello";
    // let s3 = &s2[0..2]; // Error: the slicing cannot happen inside a utf8 encoding
    let s3 = &s3[3..9]; // ok
    println!("s3: {}", s3);
}

fn test_string_iter() {
    let mut s = String::from("你好, hello");
    for c in s.chars() {
        println!("c {}", c);
    }
    for b in s.bytes() {
        println!("b {}", b);
    }
    let mut c_iter = s.chars(); // type Chats<>
    while let Some(c) = c_iter.next() {
        println!("some c {}", c);
    }
    let mut b_iter = s.bytes(); // type std::str::Bytes<>
    while let Some(b) = b_iter.next() {
        println!("some b {}", b);
    }
    // get the data through indexing
    if let Some(mut c) = s.chars().nth(4) {
        println!("4th c {}", c);
        c = '!';
    }
    // char indices
    for (i, c) in s.char_indices() {
        println!("c char indices {} {} {}", i, c, c.len_utf8());
    }
}

fn main() {
    test_string_init();
    test_string_func();
    test_string_representation();
    test_string_iter();
}
