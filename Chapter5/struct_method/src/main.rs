#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self, s: &str) -> u32 { // brief self with parameters
        println!("Rectangle area print {}", s);
        self.width * self.height
    }
}

impl Rectangle {
    fn length(self: &Self) -> u32 { // verbose self, also can be self: &Rectangle
        (self.width + self.height) * 2
    }
}

fn test_rectangle() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the {:?} is: {}", &rect1, rect1.area(&String::from("instance call")));
    println!("The area of the {:?} is: {}", &rect1, Rectangle::area(&rect1, &String::from("struct call")));
    // Error: mismatched types: expected `&Rectangle`, found `&String`
    // println!("The area of the {:?} is: {}", &rect1, Rectangle::area(&String::from("struct call"), &String::from("struct call")));
    println!("The length of the {:?} is: {}", &rect1, rect1.length());
}

fn main() {
    test_rectangle();
}
