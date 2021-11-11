use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
}

impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size); // preallocates space in the vector.
        for id in 0..size{
            workers.push(Worker::new(id, Arc.clone(&receiver))); // bump the reference count
        }
        ThreadPool{workers}
    }
    pub fn execute<F>(&self, f:F)
    where 
        F:FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id : usize,
    thread : JoinHandle<()>,
}

impl Worker{
    fn new(id : usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing...", id);
            job();
        });
        Worker{id, thread}
    }
}

pub fn handle_connection(mut stream : TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";  // b->bytes
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}