fn test_iterator_next() {
    let v = vec![String::from("a"), String::from("b"), String::from("c")];
    let mut v_iter = v.iter(); // iter is mutated in next method
    let a = v_iter.next(); println!("a {:?}", a); // Start from the first element, return Some() if exists
    let b = v_iter.next(); println!("b {:?}", b);
    let c = v_iter.next(); println!("c {:?}", c);
    let d = v_iter.next(); println!("d {:?}", d); // When out of bound, next returns None
    let e = v_iter.next(); println!("e {:?}", e); // As long as iter goes end, the next keeps returning None
}

fn test_iterator_for_each() {
    let v = vec![1, 2, 3, 4, 5];
    let mut sum: i32 = 0;
    v.iter().for_each(|o| {
        sum += o;
    });
    println!("sum = {}", sum);
    let sum2: i32 = v.iter().sum();
    println!("sum2 = {}", sum2);
}

fn test_iterator_collect() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();
    let v2: Vec<i32> = v_iter.map(|o| { o+1 }).collect();
    for o in v {
        print!("{} ", o);
    }
    println!("");
    for o in v2 {
        print!("{} ", o);
    }
    println!("");
}

fn test_iterator_filter() {
    let v = vec![1, 2, 3, 4, 5];

    // iter returns immutable reference iterator
    let v2: Vec<i32> = v.iter().filter(|&o| { o % 2 == 0 }).cloned().collect();
    for o in &v2 {
        print!("{} ", o);
    }
    println!("");
    v.iter().for_each(|&o| print!("{o} "));
    println!("");
    
    // into_iter returns depending type iterator, here returns ownership
    let v3: Vec<i32> = v.into_iter().filter(|o| { o % 2 == 0 }).collect();
    for o in &v3 {
        print!("{} ", o);
    }
    println!("");
    // v.iter().for_each(|&o| print!("{o} ")); // Error: elements of v lost ownership
    // println!("");
}

fn test_iterator_for_loop() {
    let mut v = vec![1, 2, 3, 4, 5];
    for o in v.iter_mut() {
        *o += 1;
    }
    v.iter().for_each(|o| print!("{} ", o));
    println!("");
}

fn main() {
    test_iterator_next();
    test_iterator_for_each();
    test_iterator_collect();
    test_iterator_filter();
    test_iterator_for_loop();
}
