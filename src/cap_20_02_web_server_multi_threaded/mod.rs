mod threadpool;

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

const HELLO_PATH: &str = "src/cap_20_01_web_server_single_threaded/views/hello.html";
const NOT_FOUND_PATH: &str = "src/cap_20_01_web_server_single_threaded/views/NOT_FOUND_PATH.html";

fn handle_stream(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", HELLO_PATH)
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", HELLO_PATH)
  } else {
    ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_PATH)
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

pub fn run() {
  println!("\n• cap_20_02_web_server_multi_threaded");

  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let thread_pool = ThreadPool::new(4);

  match thread_pool {
    Ok(pool) => {
      for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
          handle_stream(stream);
        });
      }
    }
    Err(e) => println!("{:?}", e),
  }
}
