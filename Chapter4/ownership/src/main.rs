fn test_scope() {
    {
        let s = "string"; // string literal, fixed content
        println!("{}", s);
        let mut string = String::from("string"); // string type put some data on heap, making the string mutable
        string.push_str(" string2");
        println!("{}", string);
    }
    // println!("{}", string); // Error: not found in the scope
}

fn test_assign() {
    let s1 = 's';
    let s2 = s1;
    println!("The value of s1 is {}", s1);
    println!("The value of s2 is {}", s2);
    test_copy_reference(s2);
    println!("The value of s2 after is {}", s2);
    let ss1 = String::from("string");
    let ss2 = ss1;
    // println!("The value of ss1 is {}", ss1); // Error: ss1 lost reference because ss1 is moved to ss2
    println!("The value of ss2 is {}", ss2);
    let ss2 = test_transfer_and_get_ownership(ss2);
    println!("The value of ss2 is {}", ss2); // Error: ss2 lost reference because ss2 is moved to function
    test_transfer_ownership(ss2);
    // println!("The value of ss2 after is {}", ss2); // Error: ss2 lost reference because ss2 is moved to function
    let sss1 = String::from("string");
    let sss2 = sss1.clone();
    println!("The value of sss1 is {}", sss1);
    println!("The value of sss2 is {}", sss2);
}

fn test_transfer_ownership(s: String) {
    println!("test_transfer_ownership {}", s);
}

fn test_transfer_and_get_ownership(s: String) -> String {
    println!("test_transfer_and_get_ownership {}", s);
    s
}

fn test_copy_reference(s: char) {
    println!("test_copy_reference {}", s);
}

fn main() {
    test_scope();
    test_assign();
}
