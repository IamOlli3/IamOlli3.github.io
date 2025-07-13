use std::net::TcpStream;
use std::io::prelude::*;
use std::net::TcpListener;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {

        let stream = stream.unwrap();

        handle_connection(stream);

    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {

        let ip = stream.peer_addr().map(|addr| addr.ip().to_string()).unwrap();

        let contents = fs::read_to_string("web.html").unwrap();

        let html = format!(r#"

         <!DOCTYPE html>
         <html>

         {}

         <div>
         <body>
         <h1>
         Thanks for your IP: {}, :)
         </h1>
         </body>
         </div>

         </html>
    
        "#, contents, ip);

        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", html.len(), html);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }

    

}

