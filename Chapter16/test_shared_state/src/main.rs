use std::sync::Mutex;
use std::sync::Arc;
// use std::rc::Rc;
// use std::cell::RefCell;
use std::thread;
use std::time::Duration;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn test_mutex_init() {
    let mutex: Mutex<Vec<i32>> = Mutex::new(vec![]); // explicit type declaration
    println!("mutex data: {:?}", mutex);
    {
        let mut v = mutex.lock().unwrap(); // block until the mutex get the exclusive permission
        println!("v type: {}", type_of(&v)); // the type is &MutexGuard<Vec<i32>>
        v.push(1); // MutexGuard implements [Deref], so we can use v directly as the operand.
        // MutexGuard implements [Drop] and releases the lock when the scope ends
    }
    println!("mutex data: {:?}", mutex);
}

fn test_mutex_multi_threads() {
    // let mutex = Rc::new(Mutex::new(0)); // Rc is not safe for thread sharing, because Rc does not implement Send
    let mutex = Arc::new(Mutex::new(0)); // Arc stands for Atomic Rc
    let mut handles = vec![];
    for i in 0..5 {
        let mutex_cp = mutex.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} try to get the mutex", i);
            thread::sleep(Duration::from_millis(1000));
            // Thread is blocked here for requiring mutex
            let mut obj = mutex_cp.lock().unwrap();
            println!("Thread {} try to modify the mutex", i);
            thread::sleep(Duration::from_millis(1000));
            *obj += 1; // Mutex provides interior mutability as RefCell
            println!("Thread {} try to release the mutex to {}", i, obj);
            drop(obj); // Mutex releases explicitly
            thread::sleep(Duration::from_millis(1000));
            println!("Thread {} ends", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Mutex value in main thread: {}", mutex.lock().unwrap());
}

// fn test_no_mutex_multi_threads() {
//     // Try to replace Mutex with RefCell
//     // Arc<T> requires T: Sync + Send + ...
//     // However RefCell does not implement Sync, so we cannot use RefCell as T of Arc
//     let no_mutex = Arc::new(RefCell::new(0));
//     let mut handles = vec![];
//     for i in 0..5 {
//         let no_mutex_cp = no_mutex.clone();
//         let handle = thread::spawn(move || {
//             println!("Thread {} try to change", i);
//             let mut obj = no_mutex_cp.borrow_mut();
//             *obj += 1;
//             println!("Thread {} change data to {}", i, obj);
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("No_mutex value in main thread: {}", no_mutex.borrow());
// }

fn test_deadlock() {
    // Even though MutexGuard release the lock when the scope ends
    // Deadlock can still happen when there are multiple mutexes
    // So it is better to use a tight scope for the data read/write range
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    let mutex1_1 = mutex1.clone();
    let mutex1_2 = mutex1.clone();
    let mutex2_1 = mutex2.clone();
    let mutex2_2 = mutex2.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let x1 = mutex1_1.lock().unwrap();
        println!("Thread 1 get mutex1 {}", x1);
        thread::sleep(Duration::from_millis(1000));
        let x2 = mutex2_1.lock().unwrap();
        println!("Thread 1 get mutex2 {}", x2);
        println!("Thread 1 over"); // Not print due to deadlock
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let x2 = mutex2_2.lock().unwrap();
        println!("Thread 2 get mutex2 {}", x2);
        thread::sleep(Duration::from_millis(1000));
        let x1 = mutex1_2.lock().unwrap();
        println!("Thread 2 get mutex1 {}", x1);
        println!("Thread 2 over"); // Not print due to deadlock
    });
    thread::sleep(Duration::from_millis(3000));
    println!("Main thread over to test deadlock");
}

fn main() {
    test_mutex_init();
    test_mutex_multi_threads();
    test_deadlock();
}
