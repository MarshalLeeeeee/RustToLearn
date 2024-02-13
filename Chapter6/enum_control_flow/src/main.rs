enum MyOption <T> {
    None,
    Some(T),
}

fn test_if_let() {
    let x: i32 = 3;
    if let y = x {
        println!("constant patter y = {}", y);
    }
    else {
        println!("constant patter match failed")
    }

    // Error: type mismatch
    // if let MyOption::Some(y) = x {
    //     println!("my option pattern y = {}", y);
    // }
    // else{
    //     println!("my option pattern match failed")
    // }

    // type must be the same for pattern match at the compile time  
    let x_some = MyOption::Some(5);
    if let MyOption::Some(y) = x_some {
        println!("my option pattern y = {}", y);
    }
    else {
        println!("my option pattern match failed")
    }
    let none: MyOption<i32> = MyOption::None; // enum generic type should be specified when type cannot be inferred from
    if let MyOption::Some(y) = none {
        println!("my option pattern y = {}", y);
    }
    else {
        println!("my option pattern match failed")
    }
}

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
    fn print_self_match(&self) {
        match self { // match control flow
            Message::MsgQuit => Message::print_quit(),
            Message::MsgMove {x, y} => Message::print_move(*x, *y),
            Message::MsgWrite(s) => Message::print_write(s),
            Message::MsgChangeColor(r, g, b) => Message::print_change_color(*r, *g, *b),
            _ => println!("other msg"),
        }
    }
    fn print_self_if_let(&self) {
        if let Message::MsgQuit = self { Message::print_quit(); }
        else if let Message::MsgMove {x, y} = self { Message::print_move(*x, *y); }
        else if let Message::MsgWrite(s) = self { Message::print_write(s); }
        else if let Message::MsgChangeColor(r, g, b) = self { Message::print_change_color(*r, *g, *b); }
        else { println!("other msg"); }
    }
}

fn test_enum_control_flow() {
    // They are all of type Message
    let msg_quit = Message::MsgQuit; msg_quit.print_self_match(); msg_quit.print_self_if_let();
    let msg_move = Message::MsgMove {x: 1, y: 2}; msg_move.print_self_match(); msg_move.print_self_if_let();
    let msg_write = Message::MsgWrite(String::from("msg_write")); msg_write.print_self_match(); msg_write.print_self_if_let();
    let msg_change_color = Message::MsgChangeColor(0, 255, 192); msg_change_color.print_self_match(); msg_change_color.print_self_if_let();
    let msg_empty = Message::MsgEmpty; msg_empty.print_self_match(); msg_empty.print_self_if_let();
}

fn main() {
    test_if_let();
    test_enum_control_flow();
}
