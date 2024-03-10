fn test_box_init() {
    println!("---- test box init ----");
    let x = 5;
    let xb = Box::new(5);
    println!("The value of x is: {}", x); // i32 performs copy rather than move when assigned
    println!("The value of xb is: {}", xb);
    let y: Box<u8> = Box::new(5);
    println!("The value of y is: {}", y); // implicit deref
    println!("The value of y is: {}", *y); // explicit deref
    let v = vec![1, 2, 3, 4, 5];
    let vb = Box::new(v);
    // println!("v: {:?}", v); // Error: Box::new takes the ownership of T, Vec<T>'s ownership is moved
    println!("vb: {:?}", vb);
}

// Fail to compile, due to recursive type
// Rust compiler error: infinite type space
// Rust compiler consider the worst space size as the type size: here is infinite
// enum List<T> {
//     Cons(T, List<T>),
//     Nil,
// }

enum List<T> {
    Cons(T, Box<List<T>>), // use Box<> as the smart pointer for List<T>, which has the known size
    Nil,
}

enum List2<'a, T> {
    Cons(T, &'a List2<'a, T>), // the reference of List2 is also okay to prevent infinite type size
    Nil,
}

impl <T: std::fmt::Display + Clone> List<T> {
    fn new_by_list(v: Vec<T>) -> List<T> {
        let sz = v.len();
        let mut l = List::Nil;
        for i in 0..sz {
            l = List::Cons(v[sz-i-1].clone(), Box::new(l)); // Transfer data from stack to heap
        }
        l
    }
    fn print_self(&self) {
        self._do_print_self();
        println!("");
    }
    fn _do_print_self(&self) {
        if let List::Cons(t, l) = self {
            print!("{t} ");
            l._do_print_self();
        }
    }
}

impl <'a, T: std::fmt::Display + Clone> List2<'a, T> {
    // Fail to compile, because the List2 is always on the stack, 
    // Thus the reference cannot live longer than the scope
    // fn new_by_list(v: Vec<T>) -> List2<'a, T> {
    //     let sz = v.len();
    //     let mut l = List2::Nil;
    //     for i in 0..sz {
    //         l = List2::Cons(v[sz-i-1].clone(), &l);
    //     }
    //     l
    // }
    fn print_self(&self) {
        self._do_print_self();
        println!("");
    }
    fn _do_print_self(&self) {
        if let List2::Cons(t, l) = self {
            print!("{t} ");
            l._do_print_self();
        }
    }
}

fn test_recursive_type() {
    println!("---- test resursive type ----");
    let l = List::new_by_list(vec![1, 2, 3, 4, 5]);
    l.print_self();
    // We cannot rely on reference to create new data with longer lifetime
    // let l2 = List2::new_by_list(vec![1, 2, 3, 4, 5]);
    // l2.print_self();
    let l2_1 = List2::Nil;
    let l2_2 = List2::Cons(1, &l2_1);
    let l2_3 = List2::Cons(2, &l2_2);
    let l2_4 = List2::Cons(3, &l2_3);
    let l2_5 = List2::Cons(4, &l2_4);
    l2_5.print_self();
}

fn test_box_mutability() {
    println!("---- test box mutability ----");
    let x = Box::new(5);
    // *x += 1; // Error: cannot mutate unmutable box
    println!("The value of x is: {}", x);
    let mut y = Box::new(5);
    *y += 1;
    println!("The value of y is: {}", y);
}

fn main() {
    test_box_init();
    test_recursive_type();
    test_box_mutability();
}
