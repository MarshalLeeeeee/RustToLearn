fn test_vector() {
    // init vector using Vec::new() with type specified
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    println!("v1 {:?}", v1);
    // init vector using vec macro with type inferred
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2 {:?}", v2);
}

fn vector_parameter1(v: &Vec<u8>) {
    println!("vector_parameter1 v {:?}", v);
}

fn vector_parameter2(v: &Vec<i32>) {
    println!("vector_parameter2 v {:?}", v);
}

fn test_vector_type() {
    let v = vec![1, 2, 3, 4, 5]; // the type of v is not decided here
    vector_parameter1(&v); // rust implicitly cast the type of v to Vec<u8> to make the code compilable
    // Error: rust has casted v to the parameter type of vector_parameter1, which conflicts with Vec<i32>
    // vector_parameter2(&v);
}

fn test_vector_func() {
    let mut _temp: i32 = 1;
    let mut v0: &mut i32 = &mut _temp;
    {
        let mut v = vec![1, 2, 3, 4, 5];
        // Get object by index
        // As we directly perform operator on v, so &v is required.
        // [] operator returns the reference of the object
        //      Error: compiler will compare the reference and its referenced object,
        //      if the referenced object has a shorter lifetime than the reference, the compile fails
        // v0 = &mut v[0];
        let v0: &mut i32 = &mut v[0];
        println!("v0 {:?}", *v0);
        *v0 = 10; // Change the value of the object by dereferencing
        println!("v0 {:?}", *v0);
        println!("v {:?}", v); // the initial vector is changed
        // Get object by get
        // As we use a method of vector, &v is optionally because auto reference is performed by rust
        // get() returns the Option of the reference of the object, index overflow is allowed by the API
        let v1: Option<&i32> = v.get(1);
        match v1 {
            Some(vo) => println!("v1 {:?}", vo),
            None => println!("v1 None"),
        }
        let v2: Option<&i32> = v.get(10);
        match v2 {
            Some(vo) => println!("v2 {:?}", vo),
            None => println!("v2 None"),
        }
        // Iterator vector and write to vector
        // for vo in & v { // Error: with & v, we have RO permission only
        for vo in &mut v {
            *vo *= 10;
        }
        println!("v {:?}", v); // the initial vector is changed
        v.pop(); // pop the last element
        println!("v {:?}", v); // the initial vector is changed
    }
    println!("v0 {:?}", v0);
}

enum EnumVectorVariant {
    IntegerVariant(i32),
    StringVariant(String),
    FloatVariant(f32),
}

impl EnumVectorVariant {
    fn incr_one(&mut self) {
        if let EnumVectorVariant::IntegerVariant(i) = self {
            *i += 1;
        }
        else if let EnumVectorVariant::FloatVariant(s) = self {
            *s += 1.0;
        }
    }
    fn print_self(&self) {
        match self {
            EnumVectorVariant::IntegerVariant(i) => println!("IntegerVariant {:?}", i),
            EnumVectorVariant::StringVariant(s) => println!("StringVariant {:?}", s),
            EnumVectorVariant::FloatVariant(s) => println!("FloatVariant {:?}", s),
        }
    }
}

fn test_vector_enum() {
    let mut v: Vec<EnumVectorVariant> = Vec::new();
    v.push(EnumVectorVariant::IntegerVariant(1));
    v.push(EnumVectorVariant::StringVariant(String::from("string_variant")));
    v.push(EnumVectorVariant::FloatVariant(1.1));
    for vv in &mut v {
        vv.incr_one();
        vv.print_self();
    }
}

fn main() {
    test_vector();
    test_vector_type();
    test_vector_func();
    test_vector_enum();
}
