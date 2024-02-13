enum Message {
    MsgQuit, // no data
    MsgMove {x: i32, y: i32}, // normal struct data
    MsgWrite(String), // type data
    MsgChangeColor(u8, u8, u8), // tuple struct data
    MsgEmpty,
}

impl Message {
    fn print_quit() {
        println!("quit");
    }
    fn print_move(x: i32, y: i32) {
        println!("move {} {}", x, y);
    }
    fn print_write(s: &String) {
        println!("write {}", s);
    }
    fn print_change_color(r: u8, g: u8, b: u8) {
        println!("change color {} {} {}", r, g, b);
    }
    fn print_self(&self) {
        match self { // match control flow
            Message::MsgQuit => Message::print_quit(),
            Message::MsgMove {x, y} => Message::print_move(*x, *y),
            Message::MsgWrite(s) => Message::print_write(s),
            Message::MsgChangeColor(r, g, b) => Message::print_change_color(*r, *g, *b),
            _ => println!("other msg"),
        }
    }
}

fn test_enum_init() {
    // They are all of type Message
    let msg_quit = Message::MsgQuit; msg_quit.print_self();
    let msg_move = Message::MsgMove {x: 1, y: 2}; msg_move.print_self();
    let msg_write = Message::MsgWrite(String::from("msg_write")); msg_write.print_self();
    let msg_change_color = Message::MsgChangeColor(0, 255, 192); msg_change_color.print_self();
    let msg_empty = Message::MsgEmpty; msg_empty.print_self();
}

enum NonNegativeOption {
    None,
    Some(i32),
}

impl NonNegativeOption {
    fn init(x: i32) -> NonNegativeOption {
        if x >= 0 {
            NonNegativeOption::Some(x)
        } else {
            NonNegativeOption::None
        }
    }
    fn get(&self) -> i32 {
        match self {
            NonNegativeOption::None => -1,
            NonNegativeOption::Some(x) => *x,
        }
    }
    fn valid(&self) -> bool {
        match self {
            NonNegativeOption::None => false,
            NonNegativeOption::Some(x) => *x >= 0,
        }
    }
}

fn test_enum_get() {
    let x = NonNegativeOption::init(5); println!("x = {}, {}", x.get(), x.valid());
    let y = NonNegativeOption::init(0); println!("y = {}, {}", y.get(), y.valid());
    let z = NonNegativeOption::init(-5); println!("z = {}, {}", z.get(), z.valid());
}

fn test_enum_assign() {
    let x = NonNegativeOption::init(5);
    let y = x;
    // println!("x = {}, y = {}", x.get(), y.get()); // Error: Move is operated for assignment
    println!("y = {}", y.get());
}

fn main() {
    test_enum_init();
    test_enum_get();
    test_enum_assign();
}
