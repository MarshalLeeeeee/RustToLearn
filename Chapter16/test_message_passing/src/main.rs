use std::sync::mpsc::{self, TryRecvError, Sender, Receiver}; // multi producer single consumer
use std::thread;
use std::time::Duration;

fn test_channel_communication() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(3000));
        let v = vec![1];
        println!("Transmit {:?}", v);
        tx.send(v).unwrap();
        println!("Has sent");
        // println!("Has sent {:?}", v); // Error, the sent data lose its ownership
        thread::sleep(Duration::from_millis(3000));
        println!("Thread over");
    });

    // recv() wait until message is received, rather than the end of the whole sub thread
    match rx.recv() {
        Ok(v) => println!("Receive {:?}", v),
        Err(e) => println!("No message received, due to {:?}", e),
    }

    // if we join the sub thread, then receive, the main thread has to wait until the completion of the sub thread
    // handle.join().unwrap();
    // match rx.try_recv() {
    //     Ok(v) => println!("Receive {:?}", v),
    //     Err(e) => println!("No message received, due to {:?}", e),
    // }
}

fn test_channel_multi_transmitter() {
    // use explicit type declaration
    let (tx1, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let tx2 = tx1.clone();
    let handle1 = thread::spawn(move || {
        let vs1 = vec![
            String::from("vs1_1"),
            String::from("vs1_2"),
            String::from("vs1_3"),
            String::from("vs1_4"),
        ];
        for s in vs1 {
            tx1.send(s).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(move || {
        let vs2 = vec![
            String::from("vs2_1"),
            String::from("vs2_2"),
            String::from("vs2_3"),
            String::from("vs2_4"),
            String::from("vs2_5"),
            String::from("vs2_6"),
        ];
        for s in vs2 {
            tx2.send(s).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    // use as an iterator to receive
    for r in rx {
        println!("Receive {}", r);
    }

    // Not equivalent to the following block
    // match rx.recv() {
    //     Ok(v) => println!("Receive {}", v),
    //     Err(e) => println!("No message received, due to {:?}", e),
    // }

    // Equivalent to the following loop block
    // loop {
    //     match rx.try_recv() {
    //         Ok(v) => println!("Receive {}", v),
    //         Err(TryRecvError::Empty) => continue,
    //         Err(TryRecvError::Disconnected) => break,
    //     }
    // }
}

fn main() {
    test_channel_communication();
    test_channel_multi_transmitter();
}
