use std::mem;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn test_unsafe_ptr() {
    // dereference of raw pointer
    // raw pointer simply records the address of the memory
    // so there is no strong relation between data and the address
    // the lifetime of two are irrelevant, and unrestricted by compiler
    // so safe rust prevent dereference of raw pointer to avoid dangling pointers
    let mut v = vec![1, 2, 3, 4, 5];
    let vr1 : *const Vec<i32> = &v; println!("type of vr1 is: {}", type_of(&vr1)); // const raw pointer
    let vr2 : *mut Vec<i32> = &mut v; println!("type of vr2 is: {}", type_of(&vr2)); // mutable raw pointer
    // dereference of raw pointer is not allowed by safe rust
    // {
    //     println!("vr1 is: {:?}", *vr1);
    //     println!("vr2 is: {:?}", *vr2);
    // }
    unsafe {
        println!("vr1 is: {:?}", *vr1); // deref of const raw ptr to read
        println!("vr2 is: {:?}", *vr2); // deref of mut raw ptr to read
        (*vr2).push(6); // deref of mut raw ptr to write
        println!("vr1 is: {:?}", *vr1); // update sync to const raw ptr
        println!("vr2 is: {:?}", *vr2); // update sync to mut raw ptr
    }

    // The following code infers why dereference of raw pointer is unsafe
    // let ptr = 0x01234usize;
    // let ptr = ptr as *mut i32;
    // unsafe { println!("ptr is: {:?}", *ptr); } // Error: STATUS_ACCESS_VIOLATION
}

// a function can be decorated with unsafe keyword when it has contract that is transparent to the compiler
// unsafe function can directly contain unsafe code
unsafe fn unsafe_func() {
    println!("unsafe func");
    let v = vec![1, 2, 3, 4, 5];
    let vr : *const Vec<i32> = &v;
    println!("vr is: {:?}", *vr);
}

fn test_unsafe_func() {
    // unsafe_func(); // unsafe func can only be called in unsafe block
    unsafe {
        unsafe_func();
    }
}

static S_COUNT: i32 = 0;
static mut COUNT: i32 = 0;

fn test_static_mut() {
    println!("S_COUNT is: {}", S_COUNT); // immutable static can be safely accessed
    // println!("COUNT is: {}", COUNT); // mutable static variable can only be accessed in unsafe block

    // Static data is stored in a single memory location
    // Read and write to a mutable static variable is like dereference a raw pointer
    // Such that Rust considers it unsafe
    unsafe {
        let sr = &COUNT;
        let srm = &mut COUNT;
        println!("COUNT is: {}", sr);
        *srm += 1;
        println!("COUNT is: {}", sr); // immutable data get changed
    }

    // global mutable static data is not thread safe
}

fn test_unsafe_trait() {
    // See unsafe impl for Send and Sync
}

// all the fields share the same memory
// only can have one field
union U {
    a: u8,
    b: f32,
    c: i32,
}

// all the fields have different memory
// all the fields exist at the same time
struct S {
    a: u8,
    b: f64,
    c: i64,
}

// all the fields share the same memory + know exactly which one is valid
// only can have one field
enum E {
    A(u8),
    B(f32),
    C(i32),
}

fn test_union() {
    // The align of struct is the bytes of the max size of the fields
    // The size of struct is the ceiling of the sum of all the data field size by the align
    println!("struct size is: {}", mem::size_of::<S>());
    println!("struct align is: {}", mem::align_of::<S>());
    // The align of enum is the bytes of the max size of one of the fields
    // The size of enum is the ceiling of the sum of the max size of one of the fields and one byte marker by the align
    println!("enum size is: {}", mem::size_of::<E>());
    println!("enum align is: {}", mem::align_of::<E>());
    // The align of union is the bytes of the max size of one of the fields
    // The size of union is the bytes of the max size of one of the fields
    // Therefore, union does not know which one is valid
    println!("union size is: {}", mem::size_of::<U>());
    println!("union align is: {}", mem::align_of::<U>());

    let u = U { b: 897.9766 };
    // Since union does not restrict the valid field
    // It is possible to 'interpret' every field to different type from the same memory.
    // It is similar to get data from one specific address as above cases, which is unsafe
    unsafe {
        println!("u.a is: {}", u.a); // translate memory to u8
        println!("u.b is: {}", u.b); // translate memory to f32
        println!("u.c is: {}", u.c); // translate memory to i32
    }
}

fn main() {
    test_unsafe_ptr();
    test_unsafe_func();
    test_static_mut();
    test_unsafe_trait();
    test_union();
}
