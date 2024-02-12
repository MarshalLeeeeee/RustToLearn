// the type does not have to be mutable if we would like to modify the struct
struct Info {
    name: String,
    age: u8,
}

// does not have field names
struct TupleStruct(String, u8);

// unit-like struct
struct UnitStruct;

fn struct_init(name: String, _age: u8) -> Info {
    Info {
        name, // shorthand is allowed only when the variable name is the same as the field name
        age: _age, // otherwise, verbose is required
    }
}

fn test_struct_init() {
    // initialize struct
    let mut info = Info {
        name: String::from("Tom"),
        age: 18,
    };
    // read data
    println!("name {}, age {}", info.name, info.age);
    // write data (if the struct instance is mutable)
    info.name = String::from("Jack");
    info.age = 20;
    println!("name {}, age {}", info.name, info.age);
    let info2 = struct_init(String::from("Mike"), 21);
    println!("name {}, age {}", info2.name, info2.age);

    // initialize tuple struct
    let mut info3 = TupleStruct(String::from("Bob"), 32);
    // read / write using index like tuple
    info3.1 = 36;
    println!("name {}, age {}", info3.0, info3.1);

    // initialize unit struct
    let info4 = UnitStruct;
}

fn test_struct_assign() {
    let info = Info {
        name: String::from("Tom"),
        age: 18,
    };
    let info2 = info; // moves the data
    // println!("name {}, age {}", info.name, info.age); // Error: cannot borrow from moved struct
    println!("name {}, age {}", info2.name, info2.age);
    let info3 = Info {
        age: 40,
        ..info2 // info2 is partially copied, used as the last
    };
    println!("name {}, age {}", info3.name, info3.age);
}

fn main() {
    test_struct_init();
    test_struct_assign();
}
