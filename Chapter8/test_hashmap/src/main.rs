use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn test_hashmap_init() {
    // init hash map with no explicit type
    let mut hash_map1 = HashMap::new();
    hash_map1.insert(String::from("apple"), 1);
    hash_map1.insert(String::from("peach"), 1);
    // hash_map1.insert(1, String::from("banana")); // Error: violate with type infer
    println!("hash map1 {:?}", hash_map1);

    // init hash map with explicit type
    let mut hash_map2: HashMap<String, i32> = HashMap::new();
    // hash_map2.insert(1, String::from("banana")); // Error: violate with type infer
    hash_map2.insert(String::from("banana"), 1);
    println!("hash map2 {:?}", hash_map2);
}

fn test_hashmap_ownership() {
    // hash map with ownership as key
    let mut hash_map3 = HashMap::new();
    let str1 = String::from("apple");
    let str2 = String::from("peach");
    hash_map3.insert(str1, 1);
    hash_map3.insert(str2, 1);
    println!("hash map3 {:?}", hash_map3);
    // Error: ownership moved
    // println!("str1 {}", str1);
    // println!("str2 {}", str2);

    // hash map with reference as key
    let mut hash_map4 = HashMap::new();
    let str1 = String::from("apple");
    let str2 = String::from("peach");
    hash_map4.insert(&str1, 1);
    hash_map4.insert(&str2, 1);
    println!("hash map4 {:?}", hash_map4);
    println!("str1 {}", str1);
    println!("str2 {}", str2);
}

fn test_hashmap_func() {
    // insert func will insert new entry or overwrite the old entry
    let mut hash_map5 = HashMap::new();
    hash_map5.insert(String::from("apple"), 1);
    hash_map5.insert(String::from("peach"), 1);
    println!("hash map5 {:?}", hash_map5);
    hash_map5.insert(String::from("apple"), 2);
    println!("hash map5 {:?}", hash_map5); // insert with the same key will overwrite the old value

    // get value with reference of key
    let get_apple = hash_map5.get(&String::from("apple"));
    println!("get_apple {:?}", get_apple); // Some(2)
    let get_banana = hash_map5.get(&String::from("banana"));
    println!("get_banana {:?}", get_banana); // None

    // entry hashmap with key
    let entry_apple = hash_map5.entry(String::from("apple"));
    println!("entry_apple {:?}", entry_apple); // OccupiedEntry
    let entry_banana = hash_map5.entry(String::from("banana"));
    println!("entry_banana {:?}", entry_banana); // VacantEntry

    // entry().or_insert() perform insert for VacantEntry
    let entry_apple_v = hash_map5.entry(String::from("apple")).or_insert(3);
    println!("entry_apple_v {}", entry_apple_v);
    let entry_banana_v = hash_map5.entry(String::from("banana")).or_insert(3);
    println!("entry_banana_v {}", entry_banana_v);
    println!("hash_map5 {:?}", hash_map5);
    // or_insert() return mutable reference
    // so modify the value with the return value from or_insert()
    let entry_apple_v_2 = hash_map5.entry(String::from("apple")).or_insert(3);
    *entry_apple_v_2 *= 10;
    println!("entry_apple_v_2 {}", entry_apple_v_2);
    let entry_banana_v_2 = hash_map5.entry(String::from("banana")).or_insert(3);
    *entry_banana_v_2 += 10;
    println!("entry_banana_v_2 {}", entry_banana_v_2);
    println!("hash_map5 {:?}", hash_map5);
    // use and_modify to perform modify function
    // use entry().and_modify().or_insert() to 
    //      first try to modify if entry exists
    //      or insert if entry does not exist
    hash_map5.entry(String::from("apple")).and_modify(|e| *e *= 10).or_insert(0);
    hash_map5.entry(String::from("orange")).and_modify(|e| *e += 10).or_insert(0);
    println!("hash_map5 {:?}", hash_map5);
}

#[derive(Debug, Eq, PartialEq)]
struct Info {
    name: String,
    age: u32,
}

impl Hash for Info {
    fn hash<H: Hasher>(&self, state: &mut H) {
        println!("person hash for {} {}", self.name, self.age);
        self.name.hash(state);
        self.age.hash(state);
    }
}

fn test_hashmap_hash() {
    let mut map: HashMap<Info, usize> = HashMap::new();
    let person1 = Info {
        name: String::from("Alice"),
        age: 30,
    };
    let person2 = Info {
        name: String::from("Bob"),
        age: 25,
    };
    map.insert(person1, 1);
    map.insert(person2, 2);
    println!("{:?}", map.get(&Info {
        name: String::from("Alice"),
        age: 30,
    }));
}

fn main() {
    test_hashmap_init();
    test_hashmap_ownership();
    test_hashmap_func();
    test_hashmap_hash();
}
