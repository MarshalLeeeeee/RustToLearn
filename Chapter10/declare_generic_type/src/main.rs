fn func_v_first<T>(v: &Vec<T>) -> Option<&T> {
    Some(v.get(0)?)
}

// fn func_v_sort<T>(v: &mut Vec<T>) {
//     v.sort(); // the trait bound `T: Ord` is not satisfied
// }

fn func_v_sort<T: std::cmp::Ord>(v: &mut Vec<T>) {
    // trait should be provided to ensure  certain function of T: `T: Ord`
    v.sort();
}

fn func_v<T: std::cmp::PartialOrd>(v: &Vec<T>) -> (&T, &T) {
    // trait should be provided to ensure  certain function of T: `T: PartialOrd`
    let mut v_min = &v[0];
    let mut v_max = &v[0];
    let sz = v.len();
    for i in 1..sz {
        if v[i] < *v_min {
            v_min = &v[i];
        }
        if v[i] > *v_max {
            v_max = &v[i];
        }
    }
    (v_min, v_max)
}

fn test_func_t() {
    let mut v_i32: Vec<i32> = vec![5, 4, 3, 2, 1];
    println!("v_32_0 is {:?}", func_v_first(&v_i32));
    let (v_min, v_max) = func_v(&mut v_i32);
    println!("v_32_min: {} v_32_max: {}", v_min, v_max);
    func_v_sort(&mut v_i32);
    println!("v_i32: {:?}", v_i32);

    let mut v_i64: Vec<i64> = vec![5, 4, 3, 2, 1];
    println!("v_64_0 is {:?}", func_v_first(&v_i64));
    let (v_min, v_max) = func_v(&mut v_i64);
    println!("v_64_min: {} v_64_max: {}", v_min, v_max);
    func_v_sort(&mut v_i64);
    println!("v_i64: {:?}", v_i64);

    let mut v_char: Vec<char> = vec!['e', 'b', 'a', 'd', 'c'];
    println!("v_char_0 is {:?}", func_v_first(&v_char));
    let (v_min, v_max) = func_v(&mut v_char);
    println!("v_char_min: {} v_char_max: {}", v_min, v_max);
    func_v_sort(&mut v_char);
    println!("v_char: {:?}", v_char);
}

struct Struct(String);

// generic type T is a parameter for struct
struct Struct_t<T> {
    x: T,
    y: T,
}

// generic type T is a parameter for impl and its trait take effects on all the methods inside
impl<T: std::fmt::Debug> Struct_t<T> {
    fn print(&self) {
        println!("x is {:?}, y is {:?}", self.x, self.y);
    }
    fn print_with<U: std::fmt::Debug>(&self, append: &U) {
        println!("x is {:?}, y is {:?}, append is {:?}", self.x, self.y, append);
    }
    // if we put get here, this method will require trait std::fmt::Debug whatsoever, even if unnecessary
    // fn get(&self) -> (&T, &T) {
    //     (&self.x, &self.y)
    // }
}

impl<T> Struct_t<T> {
    fn get(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

struct Struct_tu<T, U> {
    x: T,
    y: U,
}

fn test_struct_t() {
    let s_t_i = Struct_t { x: 5, y: 4 }; // use type infer
    s_t_i.print();
    s_t_i.print_with(&3.0_f64);
    let (s_t_i_x, s_t_i_y) = s_t_i.get();
    println!("s_t_i_x is {:?}, s_t_i_y is {:?}", s_t_i_x, s_t_i_y);
    let s_t_f: Struct_t<f32> = Struct_t { x: 5.0, y: 4.0 }; // specify generic type
    s_t_f.print();
    s_t_f.print_with(&3_i32);
    let (s_t_f_x, s_t_f_y) = s_t_f.get();
    println!("s_t_f_x is {:?}, s_t_f_y is {:?}", s_t_f_x, s_t_f_y);
    let s_t_s = Struct_t { x: Struct(String::from("hello")), y: Struct(String::from("world")) };
    // s_t_s.print(); // Error: the method exists, but its trait bounds were not satisfied
    let (s_t_s_x, s_t_s_y) = s_t_s.get();
    println!("s_t_s_x is {:?}, s_t_s_y is {:?}", s_t_s_x.0, s_t_s_y.0);
    // let s_t_m: = Struct_t { x: 5, y: 4.0 }; // Error: generic type mismatch
    let s_t_m = Struct_tu { x: 5, y: 4.0_f64 }; // OK: generic type match: T=i32, U=f64
    println!("s_t_m.x is {:?}, s_t_m.y is {:?}", s_t_m.x, s_t_m.y);
}

// use one generic type only
// fine here to construct enum.variant, but the two variants share the same set of generic types
enum EnumT<T> {
    X(T),
    Y(T),
}

impl<T> EnumT<T> {
    fn new(t: T, c: char) -> EnumT<T> {
        println!("generic type new");
        match c {
            'x' => EnumT::X(t),
            'y' => EnumT::Y(t),
            _ => panic!("Invalid char"),
        }
    }
}

impl EnumT<i32> {
    // Rust does not allow same function name with more specific type
    // Rust consider this ambiguous, and the more specific one will not be selected by compiler
    // fn new(t: i32, c: char) -> EnumT<i32> {
    //     println!("i32 type new");
    //     match c {
    //         'x' => EnumT::X(t),
    //         'y' => EnumT::Y(t),
    //         _ => panic!("Invalid char"),
    //     }
    // }
    fn new_i32(t: i32, c: char) -> EnumT<i32> {
        println!("i32 type new");
        match c {
            'x' => EnumT::X(t),
            'y' => EnumT::Y(t),
            _ => panic!("Invalid char"),
        }
    }
}

fn test_enum_t() {
    let ex_i = EnumT::new_i32(5_i32, 'x');
    let ey_f = EnumT::new(5_f32, 'y');
    let ex_c = EnumT::new('x', 'x');
    let ey_s = EnumT::new(String::from("y"), 'y');
}

fn main() {
    test_func_t();
    test_struct_t();
    test_enum_t();
}
