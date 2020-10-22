use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn web_init() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);
    //.take(2)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //判断
    // buffer

    let str = String::from_utf8(buffer.to_vec()).expect("Found invalid UTF-8");

    // println!("{}", str);
    let strurl = "./simple_poolweb/resources/";

    // let rn = b"\r\n";
    let getrequest: Vec<&str> = str.split("\r\n").collect();
    // println!("{:#?}", getrequest);
    let url: Vec<&str> = getrequest[0].split_whitespace().collect();

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     (
    //         "HTTP/1.1 200 OK\r\n\r\n",
    //         "./simple_poolweb/resources/hello.html",
    //     )
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else {
    //     (
    //         "HTTP/1.1 404 NOT FOUND\r\n\r\n",
    //         "./simple_poolweb/resources/404.html",
    //     )
    // };

    // TODO 后续要识别请求图片或者别的类型的东西

    if url.len() > 2 {
        let (mut status_line, mut filename, mut con) = (
            "HTTP/1.1 200 OK\r\n\r\n".to_string(),
            format!("{}{}", strurl, url[1]),
            "".to_string(),
        );
        if filename.ends_with("/") {
            filename = filename + "index.html";
        } else if filename.ends_with("html") {
        } else {
            filename = filename + ".html"
        }
        // println!("{}", filename);
        let contents = fs::read_to_string(filename);
        if let Ok(res) = contents {
            con = res;
        } else {
            status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
            con = fs::read_to_string("./simple_poolweb/resources/404.html").unwrap();
        }

        let response = format!("{}{}", status_line, con);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
