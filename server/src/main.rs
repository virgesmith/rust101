extern crate simple_server;

use simple_server::{Method, Server, StatusCode};

fn main() {
  let host = "127.0.0.1";
  let port = "8080";

  let server = Server::new(|request, mut response| {
    println!("Request received. {} {}", request.method(), request.uri());

    match request.method() {
      &Method::GET => {
        Ok(response.body(format!("<h1>Hi!</h1><p>GET .{}</p>\n", request.uri().path()).as_bytes().to_vec())?)
      }
      &Method::POST => {
        println!("{}", String::from_utf8_lossy(request.body()));
        Ok(response.body(format!("<h1>Hi!</h1><p>Post .{}</p>\n", request.uri().path()).as_bytes().to_vec())?)
      }
      _ => {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>\n".as_bytes().to_vec())?)
      }
    }
  });
  server.listen(host, port);
}