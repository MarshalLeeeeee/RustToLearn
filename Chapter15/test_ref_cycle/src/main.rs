use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

// enum CycleList {
//     Node(i32, Rc<CycleList>),
//     Nil,
// }

// impl CycleList {
//     fn print_self(&self) {
//         self._print_self();
//     }
//     fn _print_self(&self) {
//         match self {
//             Self::Node(i, rc) => { print!("<{}>", i); rc._print_self(); }
//             Self::Nil => { println!(""); }
//         }
//     }
//     fn set_next(&mut self, cl: &Rc<CycleList>) {
//         if let Self::Node(_, rc) = self {
//             *rc = cl.clone();
//         }
//     }
// }

enum CycleList {
    Node(i32, RefCell<Rc<CycleList>>),
    Nil,
}

impl CycleList {
    fn print_self(&self) {
        self._print_self();
    }
    fn _print_self(&self) {
        match self {
            Self::Node(i, rc) => { print!("<{}>", i); rc.borrow()._print_self(); }
            Self::Nil => { println!(""); }
        }
    }
    fn set_next(&self, cl: &Rc<CycleList>) {
        if let Self::Node(_, rc) = self {
            *rc.borrow_mut() = cl.clone();
        }
    }
}

// To create a cycle reference, we have to ensure the address is not changed after clone
// So we have to use Rc
// However, Rc cannot be ref as mutable as we would like to mutate CycleList::Node
// To solve this, we can wrap RefCell for the value we want to mutate, thus RefCell<Rc<CycleList>>
fn test_ref_cycle() {
    println!(" ---------- test_ref_cycle");
    // let mut l1 = Rc::new(CycleList::Node(1, Rc::new(CycleList::Nil)));
    // let mut l2 = Rc::new(CycleList::Node(2, l1.clone()));
    let l1 = Rc::new(CycleList::Node(1, RefCell::new(Rc::new(CycleList::Nil))));
    let l2 = Rc::new(CycleList::Node(2, RefCell::new(l1.clone())));
    l1.print_self();
    l2.print_self();
    // The following line causes a cycle reference
    l1.set_next(&l2);
    // The following print causes infinite recursion, indicating cycle ref
    // l1.print_self();
    // l2.print_self();
}

fn test_ref_weak() {
    println!(" ---------- test_ref_weak");
    let rc_1 = Rc::new(5); // use RefCell as the Target of Rc
    let rc_2 = rc_1.clone();
    let rc_3 = rc_1.clone();
    println!("The strong ref count is {}", Rc::strong_count(&rc_1));
    println!("The strong ref count is {}", Rc::weak_count(&rc_1));

    let rcw_1 = Rc::downgrade(&rc_1); // use Rc::downgrade to create Weak ref
    println!("rcw_1 is {:?}", rcw_1.upgrade());
    println!("The strong ref count is {}", Rc::strong_count(&rc_1)); // strong count stays
    println!("The strong ref count is {}", Rc::weak_count(&rc_1)); // weak count increase
    
    drop(rc_1);
    drop(rc_2);
    drop(rc_3);
    println!("rcw_1 is {:?}", rcw_1.upgrade()); // When all the strong ref goes, weak ref fails to find the actual value
}

enum WeakCycleList {
    Node(i32, RefCell<Weak<WeakCycleList>>),
    Nil,
}

impl WeakCycleList {
    fn print_self(&self) {
        self._print_self();
    }
    fn _print_self(&self) {
        match self {
            Self::Node(i, rc) => { 
                print!("<{}>", i);
                if let Some(s) = rc.borrow().upgrade() {
                    s._print_self();
                }
            }
            Self::Nil => { println!(""); }
        }
    }
    fn set_next(&self, cl: &Rc<WeakCycleList>) {
        if let Self::Node(_, rc) = self {
            *rc.borrow_mut() = Rc::downgrade(cl);
        }
    }
}

fn test_ref_cycle_with_weak() {
    let nil = Rc::new(WeakCycleList::Nil);
    let node1 = Rc::new(WeakCycleList::Node(1, RefCell::new(Rc::downgrade(&nil))));
    let node2 = Rc::new(WeakCycleList::Node(2, RefCell::new(Rc::downgrade(&nil))));
    node1.print_self();
    node2.print_self();
    
    node1.set_next(&node2);
    node2.set_next(&node1);
    // The cycle ref is still possible, however no memory leak will happen with weak ref
    // node1.print_self();
    // node2.print_self();
}

fn main() {
    test_ref_cycle();
    test_ref_weak();
    test_ref_cycle_with_weak();
}
