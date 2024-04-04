fn f(x: i32) -> i32 {
    x+1
}

fn g(x: i32) -> i32 {
    x+2
}

// Function pointer implements all traits: FnOnce, FnMut, Fn
fn function_param(f: fn(i32)->i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn closure_param<F>(f: F, arg: i32) -> i32
where F: Fn(i32)->i32 {
    f(arg) + f(arg)
}

fn test_function_and_closure_as_param() {
    let rff = function_param(f, 5);
    println!("rff {}", rff);
    let rfc = function_param(|x| { x+1 }, 5);
    println!("rfc {}", rfc);
    let rcf = closure_param(f, 5);
    println!("rcf {}", rcf);
    let rcc = closure_param(|x| { x+1 }, 5);
    println!("rcc {}", rcc);
}

fn return_function(b: bool) -> fn(i32)->i32 {
    let s = String::from("hello");
    if b {
        fn ff(x: i32) -> i32 {
            // println!("s {}", s); // Error: function cannot capture dynamic environment
            f(x) + 1
        }
        ff
    }
    else {
        // closures can only be coerced to `fn` types if they do not capture any variables
        |x| {
            // println!("s {}", s); // Capture makes the closure unable to be coerced to fn
            g(x) + 2
        }
    }
}

fn return_closure(b: bool) -> Box<dyn Fn(i32)->i32> {
    let s = String::from("hello");
    if b {
        fn ff(x: i32) -> i32 {
            println!("s hello");
            f(x) + 1
        }
        Box::new(ff)
    }
    else {
        // immutable and can be called multiple times
        Box::new(move |x| {
            println!("s {}", s);
            g(x) + 2
        })
    }
}

fn test_function_and_closure_as_return() {
    let bf1 = return_function(true)(5);
    println!("bf1 {}", bf1);
    let bf2 = return_function(false)(5);
    println!("bf2 {}", bf2);
    let bc1 = return_closure(true)(5); // use Deref of Box
    println!("bc1 {}", bc1);
    let bc2 = return_closure(false)(5); // use Deref of Box
    println!("bc2 {}", bc2);
}

fn main() {
    test_function_and_closure_as_param();
    test_function_and_closure_as_return();
}
