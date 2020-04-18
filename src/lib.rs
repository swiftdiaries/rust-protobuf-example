pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}

pub fn create_hello_request(name: String) -> greeter::HelloRequest {
    let mut hello_request = greeter::HelloRequest::default();
    hello_request.name = name;
    hello_request
}
