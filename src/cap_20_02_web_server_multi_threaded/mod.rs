use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_stream(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let hello_html_path = "src/cap_20_01_web_server_single_threaded/hello.html";
  let not_found_path = "src/cap_20_01_web_server_single_threaded/not_found_path.html";
  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", hello_html_path)
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", hello_html_path)
  } else {
    ("HTTP/1.1 404 NOT FOUND", not_found_path)
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
  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_stream(stream);
  }
}
