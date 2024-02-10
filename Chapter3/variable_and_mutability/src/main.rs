const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn test_variables() {
    println!("==== test variables ====");

    let x = 5;
    println!("The value of x is {x}");
    // fail as we assign to immutable variables
    // x = 6;
    // println!("The new value of x is {x}");

    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    println!("The new value of y is {y}");

    println!("The constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
}

fn test_shadowing() {
    println!("==== test shadowing ====");

    let z = 5;
    let z = z + 1;
    println!("The value of z is {z}");
    {
        let z = z * 2;
        println!("The value of inner z is {z}");
    }
    println!("The value of outer z is {z}");
    
    let mut w = 5;
    w = w + 1;
    println!("The value of w is {w}");
    {
        w = w * 2;
        println!("The value of inner w is {w}");
    }
    println!("The value of outer w is {w}");

}

fn main() {
    test_variables();
    test_shadowing();
}
