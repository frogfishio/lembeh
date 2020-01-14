use std::fs;
use std::fs::read;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use wasmtime::*;

fn main() {
    /******/

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let store = Store::default();
    let wasm = read("hello.wasm").expect("wasm file");
    let module = Module::new(&store, &wasm).expect("wasm module");
    let instance = Instance::new(&store, &module, &[]).expect("wasm instance");

    let answer = instance
        .find_export_by_name("answer")
        .expect("answer")
        .func()
        .expect("function");
    let result = answer.borrow().call(&[]).expect("success");
    println!("Answer: {:?}", result[0].i32());

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\n\n{} ---> {:?}", contents, result[0].i32());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
