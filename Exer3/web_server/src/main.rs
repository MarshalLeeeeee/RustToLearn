use std::{
    fs,
    thread,
    time::Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
};

struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    tx: Option<mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>>,
}

impl ThreadPool {
    fn new(num_threads: usize) -> Self {
        let mut threads = Vec::with_capacity(num_threads);
        let (tx, rx) = mpsc::channel();
        let mutex : Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>>> = Arc::new(Mutex::new(rx));
        for _ in 0..num_threads {
            let mutex_cp = mutex.clone();
            threads.push(thread::spawn(move || {
                loop {
                    let job = mutex_cp.lock().unwrap().recv().unwrap();
                    println!("Thread {:?} processes the job", thread::current().id());
                    job();
                }
            }));
        }
        Self { threads, tx:Some(tx) }
    }
    fn dispatch<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.tx.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Drop thread pool");
        drop(self.tx.take());
        for t in self.threads.drain(..) { // drain the initial threads vector
            t.join().unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let tp = ThreadPool::new(3);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        tp.dispatch( || { handle_connection(stream) });
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Thread {:?} Handle connection", thread::current().id());
    let buf_reader = BufReader::new(&mut stream);
    let mut http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // lines return Iterator<Result<String, io::Error>>
        .take_while(|line| !line.is_empty()) // http request ends with empty line
        .collect();
    println!("Thread {:?} get http request", thread::current().id());

    let mut status = "HTTP/1.1 404 NOT FOUND";
    let mut html = String::from("");
    match fs::read_to_string("html/common.html") {
        Ok(file_content) => html = file_content,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            return ;
        },
    }
    let request = http_request.remove(0);
    html = html.replace("{$REQUEST$}", &request);

    println!("Thread {:?} Handle request: {:#?}", thread::current().id(), request);
    if request.starts_with("GET / ") {
        status = "HTTP/1.1 200 OK";
        html = html.replace("{$PAGE_TITLE$}", "OK");
        html = html.replace("{$TITLE$}", "OK");
        html = html.replace("{$RESPONSE_TEXT$}", "Hello");
    }
    else if request.starts_with("GET /sleep ") {
        println!("Thread {:?} starts to sleep", thread::current().id());
        thread::sleep(Duration::from_millis(10000));
        println!("Thread {:?} wakes up", thread::current().id());
        status = "HTTP/1.1 200 OK";
        html = html.replace("{$PAGE_TITLE$}", "OK");
        html = html.replace("{$TITLE$}", "OK");
        html = html.replace("{$RESPONSE_TEXT$}", "Hello");
    }
    else {
        html = html.replace("{$PAGE_TITLE$}", "Err");
        html = html.replace("{$TITLE$}", "Err");
        html = html.replace("{$RESPONSE_TEXT$}", "404 NOT FOUND");
    }

    let length = html.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{html}");

    if let Err(e) = stream.write_all(response.as_bytes()) { // transfer byte code in &[u8]
        println!("Error writing to client: {:?}", e);
    }
    stream.flush().unwrap_or_default();
}