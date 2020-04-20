use std::io::Cursor;

use prost::Message;

pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}

pub fn create_hello_request(name: String) -> greeter::HelloRequest {
    let mut hello_request = greeter::HelloRequest::default();
    hello_request.name = name;
    hello_request
}

pub fn serialize_greeter(hello: &greeter::HelloRequest) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(hello.encoded_len());

    hello.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_greeter(buf: &[u8]) -> Result<greeter::HelloRequest, prost::DecodeError> {
    greeter::HelloRequest::decode(&mut Cursor::new(buf))
}

fn main() -> Result<(), prost::DecodeError> {
    let request = String::from("Hello, World!");

    let greeter_request = create_hello_request(request);
    let request_vector = serialize_greeter(&greeter_request);

    let request_deserialized_result = match deserialize_greeter(&request_vector) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => return Err(e),
    };
    println!("{:#?}", request_deserialized_result);
    Ok(())
}
