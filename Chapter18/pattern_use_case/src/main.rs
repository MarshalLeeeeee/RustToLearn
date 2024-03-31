fn test_match() {
    let x = Some(5);
    match x {
        Some(y) => println!("{}", y),
        None => println!("None"),
    }

    // the first one already matches
    match x {
        Some(5) => println!("Exact match 5"),
        Some(y) => println!("{}", y),
        None => println!("None"),
    }

    // the match sequence matters
    match x {
        Some(y) => println!("{}", y),
        Some(5) => println!("Exact match 5"),
        None => println!("None"),
    }

    let y: Option<i32> = None;
    match y {
        Some(y) => println!("{}", y),
        None => println!("None"),
    }

    match y {
        _ => println!("All match"),
        None => println!("None"),
    }
}

fn test_let_assign() {
    let (x, y) = (1, 2);
    println!("{}, {}", x, y);

    let p = (1, 2);
    let f = | (x,y):(i32,i32) | -> i32 { x + y }; // deform tuple to params using pattern
    let r = f(p);
    println!("{}", r);
    println!("{:?}", p);
}

fn test_refutable_pattern() {
    // let Some(x) = Some(5); // Error: x cannot be matched if right side is None, from the view of type system

    // refutable pattern can be used as condition
    // combining irrefutable pattern with condition will however raise a compile warning
    if let Some(x) = Some(5) {
        println!("{}", x);
    }
}

fn test_let_cond() {
    if let Some(x) = Some(5) {
        println!("{}", x);
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn main() {
    test_match();
    test_let_assign();
    test_let_cond();
}
