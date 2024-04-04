use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

trait T1 {
    type Item: Display;
    fn foo(&self) -> Self::Item;
    fn bar(&self, v: Self::Item) {
        println!("bar T1 {}", v);
    }
}

trait T2<T: Display> {
    fn foo(&self) -> T;
    fn bar(&self, v: T);
}

struct S;

// For associated trait implementation
// We only have to instantiate the type Item
// While the function signatures are unchanged
impl T1 for S {
    type Item = i32;
    fn foo(&self) -> i32 {
        println!("foo T1");
        0
    }
}

// For generic trait implementation with instantiated type
// Every T in the signature has to be replaced
impl T2<i32> for S {
    fn foo(&self) -> i32 {
        println!("foo T2 i32");
        0
    }
    fn bar(&self, v: i32) {
        println!("bar T2 i32 {}", v);
    }
}

// For generic trait implementation with instantiated type
// Every T in the signature has to be replaced
impl T2<f64> for S {
    fn foo(&self) -> f64 {
        println!("foo T2 f64");
        0_f64
    }
    fn bar(&self, v: f64) {
        println!("bar T2 f64 {}", v);
    }
}

fn test_trait_placeholder() {
    let s = S;

    // with associated typed trait, every api can at most have one signature
    let _ = T1::foo(&s);
    T1::bar(&s, 0);
    
    // with generic types trait, every api can possibly have multiple signatures
    let _: i32 = T2::foo(&s);
    let _: f64 = T2::foo(&s);
    T2::bar(&s, 0_i32);
    T2::bar(&s, 0_f64);
}

struct SS(i32);

impl SS {
    fn print_self(&self) {
        println!("SS {}", self.0);
    }
}

// Using default generic type as the following declaration of Trait Add
// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
impl Add for SS {
    type Output = SS;
    fn add(self, other: SS) -> SS {
        SS(self.0 + other.0)
    }
}

impl Add<i32> for SS {
    type Output = SS;
    fn add(self, other: i32) -> SS {
        SS(self.0 + other)
    }
}

fn test_trait_default_type() {
    let ss = SS(1);
    ss.print_self();
    let ss = ss + SS(2);
    ss.print_self();
    let ss = ss + 4_i32;
    ss.print_self();
}

struct SSS;

impl SSS {
    fn foo(&self) {
        println!("SSS foo");
    }
    fn bar() {
        println!("SSS bar");
    }
}

trait SSS_T1 {
    fn foo(&self) {
        println!("SSS_T1 foo");
    }
    fn bar() {
        println!("SSS_T1 bar");
    }
}
impl SSS_T1 for SSS {
    fn foo(&self) {
        println!("SSS_T1 SSS foo");
    }
    fn bar() {
        println!("SSS_T1 SSS bar");
    }
}

trait SSS_T2 {
    fn foo(&self) {
        println!("SSS_T2 foo");
    }
    fn bar() {
        println!("SSS_T2 bar");
    }
}
impl SSS_T2 for SSS {
    fn foo(&self) {
        println!("SSS_T2 SSS foo");
    }
    fn bar() {
        println!("SSS_T2 SSS bar");
    }
}

fn test_api_ambiguity() {
    let sss = SSS;
    sss.foo(); // Call the self method first.
    // by type infer of sss, compiler determines the associated method of trait for SSS is called
    SSS_T1::foo(&sss);
    <SSS as SSS_T1>::foo(&sss);
    SSS_T2::foo(&sss);
    <SSS as SSS_T2>::foo(&sss);

    SSS::bar(); // Call the self method first.
    // SSS_T1::bar(); // Error: compiler cannot infer the corresponding impl type
    <SSS as SSS_T1>::bar();
    <SSS as SSS_T2>::bar();
}

// define the bound of type using this trait
trait TT: Display {
    fn foo(&self) {
        println!("TT foo {}", self);
    }
}

struct TT_S(i32);
struct TT_S1(i32);

impl Display for TT_S {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<TT_S {}>", self.0)
    }
}

impl TT for TT_S {}
// impl TT for TT_S1 {} // Error: TT_S1 does not impl Display

fn test_trait_bound() {
    let tts = TT_S(5);
    tts.foo();
}

fn main() {
    test_trait_placeholder();
    test_trait_default_type();
    test_api_ambiguity();
    test_trait_bound();
}
