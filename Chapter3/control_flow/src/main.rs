fn test_if(x: i32) {
    // if x % 2 { // only explicit boolean expression is allowed as condition
    if x % 2 == 0 {
        println!("{x} is even number.")
    }
    else {
        println!("{x} is odd number.")
    }
}

fn test_if_expression(x: i32) {
    let x_old = x;
    // let x = if x % 2 == 0 {0}; // expression should be returned in all branches
    // let x = if x % 2 == 0 {0} else {"1"}; // expression should be of same type
    let x = if x % 2 == 0 {0} else {1};
    println!("The mapped value of {x_old} is {x}.")
}

fn fn_loop(x: i32) -> i32 {
    let mut _x = x;
    loop {
        if _x >= 10 {
            _x = _x / 10;
        }
        else {
            break _x;
        }
    }
}

fn test_loop(x: i32) {
    let high = fn_loop(x);
    println!("The high value of {x} is {high}.")
}

fn test_nested_loop() {
    let mut i = 1;
    'left_operand: loop {
        if i > 9 { break 'left_operand; }
        let mut j = 1;
        'right_operand: loop {
            if j > i { break 'right_operand; }
            let m = i * j;
            print!("{i} * {j} = {m} ");
            j = j + 1;
        }
        println!("");
        i = i + 1;
    }
}

fn test_while() {
    let mut i = 0;
    'left_operand: while i < 9 {
        println!("");
        let mut j = 0;
        i = i + 1;
        'right_operand: loop {
            j = j + 1;
            let m = i * j;
            print!("{i} * {j} = {m} ");
            if j >= i { continue 'left_operand; }
        }
    }
    println!("");

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;
    while i < 9 {
        println!("The object is {}.", array[i]);
        i = i + 1;
    }
}

fn test_for() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for o in array {
        println!("The object is {}.", o);
    }
    for number in 0..4 { // range(0, 4) in python
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 { 0 }
    else if n == 1 { 1 }
    else {
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        for _ in 0..n {
            b = a + b;
            a = b - a;
        }
        b
    }
}

fn test_for_fibonacci(n: i32) {
    let f = fibonacci(n);
    println!("The fibonacci number of {n} is {f}.")
}

fn main() {
    test_if(5);
    test_if(6);
    test_if_expression(6);
    test_loop(5712);
    test_nested_loop();
    test_while();
    test_for();
    test_for_fibonacci(6); // 0 1 1 2 3 5 8 13
}
