fn test_match_syntax() {
    // _ syntax
    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        xx => println!("anything {}", xx),
        // _ => println!("anything {}", _), // Error: _ is a reversed variable name only available for left-hand assign
    }

    // multiple pattern
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        // Equivalent to
        // 1 => println!("one or two"),
        // 2 => println!("one or two"),
        3 => println!("three"),
        xx => println!("anything {}", xx),
    }
}

fn test_range_syntax() {
    // range pattern

    // Error: open range is not allowed in pattern match
    // let x = 5;
    // match x {
    //     1..5 => println!("one to four"),
    //     xx => println!("anything {}", xx),
    // }

    // Error: expression is not allowed as range pattern
    // let x = 5;
    // match x {
    //     vec![1,2,3,4] => println!("one to four"),
    //     xx => println!("anything {}", xx),
    // }

    // Inclusive range is allowed
    let x = 5;
    match x {
        1..=5 => println!("one to five"),
        xx => println!("anything {}", xx),
    }
    
    for y in 1..5 {
        print!("<{}>", y);
    }
    println!("");
    
    for y in 'a'..='j' {
        print!("<{}>", y);
    }
    println!("");
    
    // Invalid: cross lower and upper => empty iter
    for y in 'a'..='A' {
        print!("<{}>", y);
    }
    println!("");
    
    // Invalid: reverse range => empty iter
    for y in 3..0 {
        print!("<{}>", y);
    }
    println!("");
    
    // Error: f32 not implement Step trait
    // let r: std::ops::Range<f32> = 1.0..5.0;
    // for y in r {
    //     print!("<{}>", y);
    // }
    // println!("");
}

struct S {
    x: i32,
    y: i32,
}

struct SS {
    s: S,
    v: i32,
}

#[derive(Debug)]
enum EE {
    A(i32),
    B(i32),
}

#[derive(Debug)]
enum E {
    E1(EE),
    E2(EE),
}

fn test_destructuring() {
    // destructuring
    let s = S{ x: 1, y: 2 };
    let S{ x: x_1, y: y_1 } = s;
    println!("x: {}, y: {}", x_1, y_1);

    // nested destructuring
    let ss = SS{ s: S{ x: 1, y: 2 }, v: 3 };
    let (w_1, SS{ s: S{ x: x_1, y: y_1 }, v: v_1 }) = (5, ss);
    println!("x: {}, y: {}, v: {}, w: {}", x_1, y_1, v_1, w_1);

    // partial fix match
    match s {
        S{x, y:1} => println!("S(x, 1): S({}, 1)", x),
        S{x:1, y} => println!("S(1, y): S(1, {})", y),
        S{x, y} => println!("S(x, y): S({}, {})", x, y),
        _ => println!("No match"),
    }

    // nested destructuring match
    let e = E::E1(EE::B(1));
    match e {
        E::E1(EE::A(a)) => println!("E1(EE::A(a)): {}", a),
        E::E1(EE::B(0)) => println!("E1(EE::B(0))"),
        E::E2(ee) => println!("E::E2(ee): {:?}", ee),
        E::E1(EE::B(b)) => println!("E1(EE::B(b)): {}", b),
    }

    let e2 = E::E2(EE::A(2));
    match e2 {
        E::E1(EE::A(a)) => println!("E1(EE::A(a)): {}", a),
        E::E1(EE::B(0)) => println!("E1(EE::B(0))"),
        E::E2(ee) => println!("E::E2(ee): {:?}", ee),
        E::E1(EE::B(b)) => println!("E1(EE::B(b)): {}", b),
    }

    let tuple = (1, 2, 3);
    if let (head, .., tail) = tuple {
        println!("head: {}, tail: {}", head, tail);
    }
    if let (head, ..) = tuple {
        println!("head: {}", head);
    }
    if let (.., tail) = tuple {
        println!("tail: {}", tail);
    }
    // Error, .. can only be used in once in a pattern
    // if let (.., mid, ..) = tuple {
    //     println!("mid: {}", mid);
    // }
}

fn test_underline_syntax() {
    let s = Some(String::from("hello"));

    // Error: _s will take the ownership of s
    // if let Some(_s) = s {
    //     println!("Some: {:?}", _s);
    // }
    // println!("Revisit: {:?}", s);
    // Equivalent to the question below
    // let _s = s;
    // println!("Revisit: {:?}", s);

    // With explicit ref, _s will only borrow from s
    if let Some(ref _s) = s {
        println!("Some: {:?}", _s);
    }
    println!("Revisit: {:?}", s);

    // Since _ will never be read, _ will never take the ownership
    if let Some(_) = s {
        println!("Some match");
    }
    println!("Revisit: {:?}", s);
    let _ = s; // Alike here, _ will never take the ownership
    println!("Revisit: {:?}", s);
}

fn test_match_guard() {
    let s = Some(4);
    match s {
        Some(x) if x % 2 == 0 => println!("Even some"),
        Some(x) => println!("Odd some"),
        _ => println!("No match"),
    }

    let x = 4;
    match x {
        4 | 5 | 6 if false => println!("Match"),
        (4 | 5 | 6) if false => println!("Match"), // Equivalent expression, the match guard is applies to the whole pattern
        _ if false => println!("False no match"),
        _ => println!("No match"),
    }
}

fn test_binding() {
    // bind a variable to an anonymous pattern
    let s = S{x: 1, y: 2};
    match s {
        // S{x: 0..=2, y: 0..=2} => println!("S{{x: 0..=2, y: 0..=2}}, {} {}", x, y), // Error: x, y here is not a variable
        S{x: x_1 @ 0..=2, y: y_1 @ 0..=2} => println!("S{{x: 0..=2, y: 0..=2}}, {} {}", x_1, y_1), // Error: x, y here is not a variable
        _ => println!("No match"),
    }
}

fn main() {
    test_match_syntax();
    test_range_syntax();
    test_destructuring();
    test_underline_syntax();
    test_match_guard();
    test_binding();
}
