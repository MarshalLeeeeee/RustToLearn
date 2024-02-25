use std::error::Error;

fn test_closure_ownership() {
    println!("--------- test_closure_ownership ---------");
    
    let v1 = vec![1, 2, 3];
    println!("[pre check] v1 : {:?}", v1);
    let c1 = || { println!("[c1] v1: {:?}", v1); };
    println!("[mid check] v1 : {:?}", v1);
    c1();
    println!("[valid check] v1 : {:?}", v1);
    
    let mut v2 = vec![1, 2, 3];
    println!("[pre check] v2 : {:?}", v2);
    let mut c2 = || { v2.push(4); println!("[c2] v2: {:?}", v2); };
    // println!("[mid check] v2 : {:?}", v2); // Error: reference overlap with mutable reference
    c2();
    println!("[valid check] v2 : {:?}", v2);
    
    let mut v3 = vec![1, 2, 3];
    println!("[pre check] v3 : {:?}", v3);
    let mut c3 = move || { v3.push(4); println!("[c3] v3: {:?}", v3); };
    // println!("[mid check] v3 : {:?}", v3); // Error: ownership of v3 moved to closure
    c3();
    // println!("[valid check] v3 : {:?}", v3); // Error: ownership of v3 moved to closure
    println!("[valid check] v3 : False");
    
    let mut v4 = vec![1, 2, 3];
    let mut c4 = move || { v4.push(4); println!("[c4] v4: {:?}", v4); }; // move happens right in declaration of c4
    // println!("[pre check] v4 : {:?}", v4); // Error: ownership of v4 moved to closure
    println!("[valid check] v4 : False");

    println!("--------- test_closure_ownership ---------");
}

fn closure_once<F>(f: F)
where F: FnOnce(String, i32) -> Result<(), Box<dyn Error>> {
    f("FnOnce".to_string(), 4);
}

fn closure_mut<F>(mut f: F)
where F: FnMut(String, i32) -> Result<(), Box<dyn Error>> {
    f("FnMut".to_string(), 5);
}

fn closure_<F>(f: F)
where F: Fn(String, i32) -> Result<(), Box<dyn Error>> {
    f("Fn".to_string(), 6);
}

struct ClosureStruct {
    v: Vec<i32>,
}

impl ClosureStruct {
    fn closure(&mut self, s: String, x: i32) -> Result<(), Box<dyn Error>> {
        self.v.push(x);
        print!("[{}] ", s);
        println!("[ClosureStruct] self.v: {:?}", self.v);
        Ok(())
    }
}

fn test_closure_param() {
    println!("--------- test_closure_param ---------");
    
    // immutable borrow
    let v_im_ref = vec![1, 2, 3];
    let c_im_ref = |s, _| {
        print!("[{}] ", s);
        println!("[c_im_ref] v: {:?}", v_im_ref);
        Ok(())
    };
    closure_(c_im_ref);
    closure_mut(c_im_ref);
    closure_once(c_im_ref);
    
    // mutable borrow
    let mut v_m_ref = vec![1, 2, 3];
    let mut c_m_ref = |s, x| {
        v_m_ref.push(x);
        print!("[{}] ", s);
        println!("[c_m_ref] v: {:?}", v_m_ref);
        Ok(())
    };
    // closure_(c_m_ref); // Error: closure not implement Fn, because closure has mutable value
    closure_mut(c_m_ref);
    // closure_once(c_m_ref); // Error: closure moved, because mutable closure moves ownership in assignment
    
    let mut v_move = vec![1, 2, 3];
    let mut c_move = move |s, x| {
        v_move.push(x);
        print!("[{}] ", s);
        println!("[c_move] v: {:?}", v_move);
        Ok(())
    };
    // closure_(c_move); // Error: closure not implement Fn, because closure has mutable value
    closure_mut(c_move); // Although use move, this closure can still be called multiple times
    // closure_once(c_move);
    
    let mut v_move2: Vec<String> = Vec::new();
    let ss = "s".to_string();
    let mut c_move2 = move |s, _| {
        v_move2.push(ss);
        print!("[{}] ", s);
        println!("[c_move2] v: {:?}", v_move2);
        Ok(())
    };
    // closure_mut(c_move2); // Error: closure not implement FnMut, because ss cannot lose ownership multiple times
    closure_once(c_move2);
    
    let mut v_struct_like = vec![1, 2, 3];
    let mut c_struct_like = ClosureStruct { v: v_struct_like };
    c_struct_like.closure("c_struct_like".to_string(), 4);
    c_struct_like.closure("c_struct_like".to_string(), 5); // transfer ownership, but can still be called multiple times
    // println!("v_struct_like: {:?}", v_struct_like); // Error: moved value: `v_struct_like`

    println!("--------- test_closure_param ---------");
}

fn main() {
    test_closure_ownership();
    test_closure_param();
}
