use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
extern crate simple_server;

use simple_server::{Method, Server, StatusCode};

fn main() {
  let host = "localhost";
  let port = 8080;

  let server = Server::new(|request, mut response| {
    println!("Request received. {} {}", request.method(), request.uri());

    match request.method() {
      &Method::GET => {
        let filename = format!("public{}", request.uri().path());
        if Path::new(&filename).is_file() {
          let mut file = File::open(filename)?;  //.unwrap());
          let mut content = Vec::new();
          // read the whole file
          file.read_to_end(&mut content)?;
          Ok(response.body(content)?)
        } else { 
          response.status(StatusCode::NOT_FOUND);
          Ok(response.body("<h1>404</h1><p>Not found!<p>\n".as_bytes().to_vec())?)
        }
      }
      &Method::POST => {
        let mut f = File::create(format!("public{}", request.uri().path()))?;
        // TODO file-specific mutex
        let _lock = Arc::new(Mutex::new(0));
        f.write_all(request.body())?;
        f.sync_data()?;
        Ok(response.body(format!("<p>POST: .{}</p>\n", request.uri().path()).as_bytes().to_vec())?)
      }
      _ => {
        response.status(StatusCode::FORBIDDEN);
        Ok(response.body("<h1>403</h1><p>Not found!<p>\n".as_bytes().to_vec())?)
      }
    }
  });
  println!("Server at {} listening on port {}", host, port);
  server.listen(host, &port.to_string());
}