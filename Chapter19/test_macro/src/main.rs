use test_macro::my_module::my_module;
use test_macro::MyTrait2;
use test_macro::attr_macro;
use test_macro::func_macro;

// generate code by code pattern matching and replacement
fn test_declarative_macro() {
    let v = my_module::new_vector();
    println!("{:?}", v);
    let v2 = test_macro::my_vec!{1;2;3}; // it does not matter, if we use () [] or {} after the macro!
    println!("{:?}", v2);
    let v3 = my_module::new_vector_range(1,7);
    println!("{:?}", v3);
}

struct S1;
impl MyTrait2 for S1 {
    fn mt2_f(&self) {
        println!("S1::mt2_f");
    }
}

#[derive(MyTrait2)]
struct S2;

fn test_custom_derive_macro() {
    let s1 = S1;
    s1.mt2_f();
    let s2 = S2;
    s2.mt2_f();
}

#[attr_macro]
fn test_attribute_like_macro() {
    println!("test_attribute_like_macro");
}

fn test_function_like_macro() {
    let v = func_macro!{4 * 2};
    func_macro!(println!("{:?}", v));
}

// generate code by manipulating the input code
fn test_procedural_macro() {
    test_custom_derive_macro();
    test_attribute_like_macro();
    test_function_like_macro();
}

fn main() {
    test_declarative_macro();
    test_procedural_macro();
}
