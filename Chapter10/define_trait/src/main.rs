trait Trait<T: std::fmt::Display> {
    fn new(t: T) -> Self;
    fn format(&self) -> String {
        format!("Default format")
    }
}

trait Trait2 {
    fn print_self(&self);
}

#[derive(Debug, Clone)]
struct Struct<T> {
    field: T,
}

impl<T: std::fmt::Display> Trait<T> for Struct<T> {
    fn new(t: T) -> Self {
        Self { field:t }
    }
    fn format(&self) -> String {
        format!("The field is {}", self.field)
    }
}

impl<T: std::fmt::Debug> Trait2 for Struct<T> {
    fn print_self(&self) {
        println!("print self {:?}", self);
    }
}

struct Struct2 {
    field: usize,
}

impl Trait<usize> for Struct2 { // specify generic type for Trait
    fn new(t: usize) -> Self {
        Self { field:t }
    }
}

fn test_trait() {
    let s_i32 = Struct::new(1); // infer generic type
    println!("{}", s_i32.format());
    let s_i8 = Struct::<i8>::new(2); // explicit generic type
    println!("{}", s_i8.format());
    let s_c = Struct::<char>::new('c');
    println!("{}", s_c.format());
    let s_usize = Struct2::new(3);
    println!("{}", s_usize.format()); // use default implementation of format
}

fn trait_param1<T: Trait<usize>>(s: &T) {
    println!("format to {}", s.format());
}

fn trait_param1_1<T: std::fmt::Display>(s: &impl Trait<T>) { // use impl keyword to use generic trait
    println!("format to {}", s.format());
}

fn trait_param2<T: Trait2>(s: &T) {
    s.print_self();
}

fn trait_param2_1(s: &impl Trait2) { // impl defines the set of types with target behavior
    s.print_self();
}

fn trait_param2_2<T>(s: &T)
where 
    T:Trait2 { // using where keyword for readability
    s.print_self();
}

// Error: Although impl Trait2 means some type that has Trait2
// the compiler does not allow different types
// in other word, the specific type is monomorphized to one type in compile time
// fn trait_param3(b: bool, s: &(impl Trait2 + Clone)) -> impl Trait2 { // return types has Trait2
//     if b { (*s).clone() }
//     else { Struct{field: 3_i32} }
// }

fn trait_param3(b: bool, s: &(impl Trait2 + Clone)) -> impl Trait2 { // return types has Trait2
    (*s).clone()
}

fn test_trait_param() {
    let s = Struct::new(1);
    trait_param1(&s);
    trait_param1_1(&s);
    trait_param2(&s);
    trait_param2_1(&s);
    trait_param2_2(&s);
    let s_1 = trait_param3(true, &s);
    trait_param2(&s_1);
    let s_0 = trait_param3(false, &s);
    trait_param2(&s_0);
}

fn main() {
    test_trait();
    test_trait_param();
}
