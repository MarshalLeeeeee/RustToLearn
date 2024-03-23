use std::rc::Rc;
use std::cell::RefCell;

fn test_ref_cell_mutability() {
    println!(" ---------- test_ref_cell_mutability");

    // box is mutable
    let mut b = Box::new(5);
    *b += 1;
    println!("new b is {}", b);
    
    // rc is always immutable
    let mut rc = Rc::new(5);
    // *rc += 1; // Error: cannot assign
    println!("new rc is {}", rc);
    
    // ref cell
    let ref_cell = RefCell::new(5);
    *ref_cell.borrow_mut() += 1; // even if the reference is immutable, we can borrow its mutable reference
    // println!("new ref_cell is {}", ref_cell); // Error: RefCell<T> cannot be dereferenced
    println!("new ref_cell is {}", ref_cell.borrow()); // we have to use borrow of RefCell<T> to get its value
}

fn test_rc_mutability_with_ref_cell() {
    println!(" ---------- test_rc_mutability_with_ref_cell");
    
    let rc_mut_1 = Rc::new(RefCell::new(5)); // use RefCell as the Target of Rc
    let rc_mut_2 = rc_mut_1.clone();
    let rc_mut_3 = rc_mut_1.clone();
    println!("rc_mut_1: {}, rc_mut_2: {}, rc_mut_3: {}", rc_mut_1.borrow(), rc_mut_2.borrow(), rc_mut_3.borrow());
    *rc_mut_1.borrow_mut() += 1; // We can now mutate the value, even if Rc is immutable
    println!("rc_mut_1: {}, rc_mut_2: {}, rc_mut_3: {}", rc_mut_1.borrow(), rc_mut_2.borrow(), rc_mut_3.borrow());
}

fn type_of<T>(_: & T) -> &'static str {
    std::any::type_name::<T>()
}

fn test_ref_cell_borrow_type() {
    println!(" ---------- test_ref_cell_borrow_type");
    let rfc = RefCell::new(5);
    println!("The type of borrow {}", type_of(&rfc.borrow())); // The type of borrow is Ref<T>
    println!("The type of mut borrow {}", type_of(&rfc.borrow_mut())); // The type of borrow is RefMut<T>
}

fn test_ref_cell_counting() {
    println!(" ---------- test_ref_cell_counting");
    // Normal ref recap
    let mut v = vec![1];
    let vr1 = &v;
    println!("vr1: {:?}", vr1);
    let vr2 = &v;
    println!("vr2: {:?}", vr2);
    let mut vr3 = &mut v;
    println!("vr3: {:?}", vr3);
    vr3.push(2);
    println!("vr3: {:?}", vr3);
    // println!("vr2: {:?}", vr2); // Error: mutable ref intersects with immutable ref in actual life span
    println!("v: {:?}", v);

    // RefCell borrow rule
    let rfc = RefCell::new(5);
    // Error: The following will not cause a compile error, because both rfc_borrow and rfc_borrow_mut are immutable for compiler
    // However, at run time, RefCell reject by panic.
    // RefCell only allows multiple immutable borrows or only one mutable borrow considering the whole life span.
    // This is unlike the way in normal ref checked by the compiler, where the actual life span is taken as the evidence.
    // let rfc_borrow = rfc.borrow();
    // let rfc_borrow_mut = rfc.borrow_mut();
    
    // The following block is okay, because the life time of two are not intersected.
    let rfc_borrow = rfc.borrow();
    println!("The borrow {}", rfc_borrow);
    drop(rfc_borrow); // Explicit drop to allow the later borrow
    let rfc_borrow_mut = rfc.borrow_mut();
    println!("The borrow_mut {}", rfc_borrow_mut);
    drop(rfc_borrow_mut); // Explicit drop to allow the later borrow
    
    // Multiple immutable borrows are allowed.
    let rfc_borrow_1 = rfc.borrow();
    let rfc_borrow_2 = rfc.borrow();
    let rfc_borrow_3 = rfc.borrow();
    println!("The borrow_1 {}, borrow_2 {}, borrow_3 {}", rfc_borrow_1, rfc_borrow_2, rfc_borrow_3);
}

fn main() {
    test_ref_cell_mutability();
    test_rc_mutability_with_ref_cell();
    test_ref_cell_borrow_type();
    test_ref_cell_counting();
}
