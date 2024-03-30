use std::thread;
use std::time::Duration;

fn counting(n: i32, s: &str, t: u64) {
    for i in 1..n {
        println!("Count to {} from [{}]", i, s);
        // Release the control of the executing thread for at least t milliseconds
        thread::sleep(Duration::from_millis(t));
    }
}

fn test_thread_init() {
    let handle = thread::spawn(|| { counting(20, "sub thread", 100); });
    counting(7, "main thread", 200);
    // When the main thread completes, all sub threads are shut down and recycled
    // It is reasonable, as the variable [handle] is dropped with the thread owned released as well.
}

fn test_thread_join() {
    let handle1 = thread::spawn(|| { counting(5, "sub thread1 with join", 10); });
    let handle2 = thread::spawn(|| { counting(5, "sub thread2 with join", 10); });
    // Guarantee the main thread will blocked until all joined thread finishes.
    handle1.join().unwrap();
    handle2.join().unwrap();
    // handle.join().unwrap(); // Error: join() moves the ownership of the JoinHandle, therefore cannot be double called
    // The following print flushes when all the joined threads are completed
    // However the unjoined threads are not taken considerations for this blocking
    println!("After threads join (and complete)");
}

fn test_thread_ownership() {
    let v = vec![1,2,3,4,5];
    // let handle = thread::spawn(|| {
    //     println!("{:?}", v);
    // });
    // drop(v); // Possible logic that make closure borrowing v outlive the lifetime of v before handle join the main thread

    // Use move to move the ownership of v to the thread
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();
}

fn test_thread_panic() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(50));
        panic!("Panic in thread"); // panic terminate the current thread only.
    });
    thread::sleep(Duration::from_millis(100));
    println!("Main thread survive?");
}

fn main() {
    test_thread_init();
    test_thread_join();
    test_thread_ownership();
    test_thread_panic();
    println!("Almost the end of the main thread");
}
