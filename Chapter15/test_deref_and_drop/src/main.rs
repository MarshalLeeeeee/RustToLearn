use std::ops::{Deref, DerefMut};

fn type_of<T>(_: & T) -> &'static str {
    std::any::type_name::<T>()
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}

// Deref is only implemented for &T by default
// For customer type T, only by implement Deref can we use *T to deref
impl<T: std::fmt::Display> Deref for MyBox<T> {
    type Target = T; // define the associated type
    fn deref(&self) -> &Self::Target {
        println!("deref for {}", self.0);
        &self.0 // return the reference of the Target
    }
}

impl<T: std::fmt::Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("deref mut for {}", self.0);
        &mut self.0 // return the reference of the Target
    }
}

fn test_deref_syntax() {
    println!("--- test deref syntax ---");
    let x = 5;
    let x_ref_1 = &x; // &i32
    let x_ref_2: *const i32 = &x; // *const i32
    println!("x_ref_1 {:?} {}", x_ref_1, type_of(&x_ref_1));
    println!("x_ref_2 {:?} {}", x_ref_2, type_of(&x_ref_2));
    let y = 5;
    let yb = Box::new(y);
    println!("yb {}", *yb);
    let ymb = MyBox::new(y);
    println!("ymb {}", *ymb); // Since Deref is implemented, we can deref MyBox using * 
    println!("ymb {}", *(ymb.deref())); // De-sugar for *ymb syntax
}

fn print_str(s: &str) {
    println!("print str: {}", s);
}

fn test_deref_coercion() {
    println!("--- test deref coercion ---");
    let s = MyBox::new("hello");
    println!("The type of s is {}", type_of(&s)); // test_deref_and_drop::MyBox<&str>
    // Deref coercion is implemented here
    // the compiler can turn &T into &U, if T implements Deref<Target=U>
    print_str(&s);
    print_str(&(*s)); // de-sugar for deref coercion
}

fn test_deref_mutability() {
    println!("--- test deref mutability ---");
    let x = 5;
    let mut xb = MyBox::new(x);
    *xb += 1; // call deref mut here
    println!("xb {}", *xb);
}

struct MyBox2<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> MyBox2<T> {
    fn new(t: T) -> Self {
        MyBox2(t)
    }
}

impl<T: std::fmt::Display> Deref for MyBox2<T> {
    type Target = T; // define the associated type
    fn deref(&self) -> &Self::Target {
        &self.0 // return the reference of the Target
    }
}

impl<T: std::fmt::Display> Drop for MyBox2<T> {
    fn drop(&mut self) { // called when the lifetime is over
        println!("Drop for MyBox2<T> {}", self.0);
    }
}

fn test_drop_syntax() {
    println!("--- test drop syntax ---");
    let x = 5;
    let xb = MyBox2::new(x);
    println!("xb {}", *xb);
    let y = 6;
    let yb = MyBox2::new(y);
    println!("yb {}", *yb);
    // yb.drop(); // Error: Explicit destructor is not allowed
    drop(yb); // yb is dropped here explicitly
    // println!("yb {}", *yb); // Error: yb lose ownership after drop
    println!("--- end of test drop syntax ---");
    // the implicit drop for xb is called here
}

fn main() {
    test_deref_syntax();
    test_deref_coercion();
    test_deref_mutability();
    test_drop_syntax();
}
