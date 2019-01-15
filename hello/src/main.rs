// web server example from the online rust book 2ed
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;

extern crate threadpool;
use threadpool::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| { handle_connection(stream); });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  let get = "GET";// / HTTP/1.1\r\n";
  //let post = "POST"; // / HTTP/1.1\r\n";
  let req = String::from_utf8_lossy(&buffer);
  let mut header = req.lines().next().unwrap().split(' ');
  let cmd = header.next().unwrap();
  let file = format!(".{}", header.next().unwrap());
  println!("{}:{}", cmd, file);

  let (status_line, filename) = if cmd == get && Path::new(&file).is_file() {
  // if file exists...
    ("HTTP/1.1 200 OK\r\n\r\n", file)
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./404.html".to_owned())
  };
  // } else if cmd == post {
  //   ("HTTP/1.1 200 OK\r\n\r\n", "post.html")
  // } else {
  //   ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  
  println!("{}", filename);
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let response = format!("{}{}", status_line, contents);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();

}