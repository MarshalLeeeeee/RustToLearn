#[derive(Clone)]
enum ListBox {
    Cons(i32 , Box<ListBox>),
    Nil,
}

impl ListBox {
    fn print_self(&self) {
        match self{
            Self::Cons(v, _) => println!("{}", v),
            Self::Nil => println!("Nil"),
        }  
    }
}

use std::rc::Rc;

#[derive(Clone)]
enum ListRc {
    Cons(i32 , Rc<ListRc>),
    Nil,
}

impl ListRc {
    fn print_self(&self) {
        match self{
            Self::Cons(v, _) => println!("{}", v),
            Self::Nil => println!("Nil"),
        }  
    }
}

fn type_of<T>(_: & T) -> &'static str {
    std::any::type_name::<T>()
}

fn test_rc_syntax() {
    // Stack data can only be owned by one variable
    println!("--- box new");
    let box_new_1 = ListBox::Cons(1, Box::new(ListBox::Nil));
    box_new_1.print_self();
    let box_new_2 = ListBox::Cons(2, Box::new(box_new_1)); // box_new_1 is moved here
    box_new_2.print_self();
    // box_new_1.print_self(); // Error: box_new_1 is box_new_1 stack data, which has lost ownership
    
    // Rc::new takes ownership of the data, same as Box::new
    println!("--- rc new");
    let rc_new_1 = ListRc::Cons(1, Rc::new(ListRc::Nil));
    rc_new_1.print_self();
    let rc_new_2 = ListRc::Cons(2, Rc::new(rc_new_1)); // rc_new_1 is moved here
    rc_new_2.print_self();
    // rc_new_1.print_self(); // Error: rc_new_1 is rc_new_1 stack data, which has lost ownership
    
    // Box::clone, similar to unique_ptr in cpp
    println!("--- box clone");
    let box_clone_1 = Box::new(ListBox::Cons(1, Box::new(ListBox::Nil)));
    box_clone_1.print_self();
    let box_clone_2 = ListBox::Cons(2, box_clone_1.clone()); // box_clone_1 is clone here
    box_clone_2.print_self();
    box_clone_1.print_self();
    let box_clone_3 = Box::new(5); println!("type of box_clone_3 {}", type_of(&box_clone_3));
    let box_clone_4 = box_clone_3.clone(); println!("type of box_clone_4 {}", type_of(&box_clone_4));
    let box_clone_5 = box_clone_3.clone(); println!("type of box_clone_5 {}", type_of(&box_clone_5));
    // the stack addresses are different, and heap addresses are different as well
    let box_clone_3_ref: *const Box<i32> = &box_clone_3;
    println!("ref of box_clone_3 {:?} {:p}", box_clone_3_ref, box_clone_3);
    let box_clone_4_ref: *const Box<i32> = &box_clone_4;
    println!("ref of box_clone_4 {:?} {:p}", box_clone_4_ref, box_clone_4);
    let box_clone_5_ref: *const Box<i32> = &box_clone_5;
    println!("ref of box_clone_5 {:?} {:p}", box_clone_5_ref, box_clone_5);
    
    // Rc::clone, similar to shared_ptr in cpp
    println!("--- rc clone");
    let rc_clone_1 = Rc::new(ListRc::Cons(1, Rc::new(ListRc::Nil)));
    rc_clone_1.print_self();
    let rc_clone_2 = Rc::new(ListRc::Cons(2, rc_clone_1.clone())); // rc_clone_1 is clone here
    rc_clone_2.print_self();
    rc_clone_1.print_self();
    // when using rc:clone, the reference count increases instead of allocating new space on the heap
    let rc_clone_3 = Rc::new(5); println!("type of rc_clone_3 {}, rc: {}", type_of(&rc_clone_3), Rc::strong_count(&rc_clone_3));
    let rc_clone_4 = rc_clone_3.clone(); println!("type of rc_clone_4 {}, rc: {}", type_of(&rc_clone_4), Rc::strong_count(&rc_clone_3));
    let rc_clone_5 = rc_clone_3.clone(); println!("type of rc_clone_5 {}, rc: {}", type_of(&rc_clone_5), Rc::strong_count(&rc_clone_3));
    // the stack addresses are different, however heap addresses are same
    let rc_clone_3_ref: *const Rc<i32> = &rc_clone_3;
    println!("ref of rc_clone_3 {:?} {:p}", rc_clone_3_ref, rc_clone_3);
    let rc_clone_4_ref: *const Rc<i32> = &rc_clone_4;
    println!("ref of rc_clone_4 {:?} {:p}", rc_clone_4_ref, rc_clone_4);
    let rc_clone_5_ref: *const Rc<i32> = &rc_clone_5;
    println!("ref of rc_clone_5 {:?} {:p}", rc_clone_5_ref, rc_clone_5);
}

fn test_rc_mutability() {
    println!("--- rc mutability");
    let mut rc_mut_1 = Rc::new(5);
    let mut rc_mut_2 = rc_mut_1.clone();
    let mut rc_mut_3 = rc_mut_1.clone();
    // *rc_mut_3 += 1; // Error: rc is always immutable
    println!("rc_mut_1: {}, rc_mut_2: {}, rc_mut_3: {}", rc_mut_1, rc_mut_2, rc_mut_3);
}

fn main() {
    test_rc_syntax();
    test_rc_mutability();
}
