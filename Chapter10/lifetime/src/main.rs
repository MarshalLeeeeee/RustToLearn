#[derive(Debug)]
struct Str(String);

trait StrLen {
    fn len(&self) -> usize;
}

impl StrLen for Str {
    fn len(&self) -> usize {
        self.0.len()
    }
}

fn test_lifetime() {
    let s1 = Str(String::from("hello"));
    let mut s1_ref = &s1;
    {
        let s2 = Str(String::from("world"));
        let mut s2_ref = &s2;
        // s1_ref = s2_ref; // Error: s2's lifetime is not longer than s1_ref
    }
    println!("{:?}", s1_ref);
}

// Error: compiler does not know if the return reference is s1 or s2
// fn lifetime_param(s1: &Str, s2: &Str) -> &Str

// Generic lifetime
// the function accept parameter with any lifetime
// reference decorated with same lifetime annotation will be substituted with the common lifetime
// which is the contract lifetime of the return reference
// generic lifetime always comes prior to generic type
fn lifetime_param<'a, T: StrLen>(s1: &'a T, s2: &'a T) -> &'a T {
    if s1.len() < s2.len() { s1 }
    else { s2 }
}

fn test_lifetime_param() {
    let s1 = Str(String::from("hello"));
    let mut s1_ref = &s1;
    let mut s3_ref = &s1;
    {
        let s2 = Str(String::from("world"));
        let s2_ref = &s2;
        // s1_ref = lifetime_param(s1_ref, s2_ref); // Error: the common lifetime is shorter than what the return reference expected
        s1_ref = lifetime_param(s1_ref, s3_ref);
    }
    println!("{:?}", s1_ref);
}

// Error: the lifetime is unrelated between struct and its field
// struct StrRef {
//     s: &String,
// }

struct StrRef<'a> {
    s: &'a String,
}

fn test_lifetime_struct() {
    let s = String::from("hello");
    let sr = StrRef { s: &s };
    println!("sr hold {}", sr.s);
}

enum StrEnum<'a> {
    S1(&'a String),
    S2(&'a str),
}

// use elided lifetime as the specified lifetime for generic lifetime
impl StrEnum<'_> {
    fn get_self(&self) -> &str {
        match self {
            StrEnum::S1(s) => s.as_str(),
            StrEnum::S2(s) => s,
        }
    }
}
// Equivalent to:
// impl<'a> StrEnum<'a> {
//     fn get_self(&self) -> &'a str {
//         match self {
//             StrEnum::S1(s) => s.as_str(),
//             StrEnum::S2(s) => s,
//         }
//     }
// }

fn test_lifetime_enum() {
    let ss = "world";
    {
        let s = String::from("hello");
        let s1 = StrEnum::S1(&s);
        let s2 = StrEnum::S2(&ss);
        println!("s1: {}, s2: {}", s1.get_self(), s2.get_self());
    }
}

fn main() {
    test_lifetime();
    test_lifetime_param();
    test_lifetime_struct();
    test_lifetime_enum();
}
