extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

use jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::{ServerBuilder};

mod server {
    pub fn start() {
        let mut io = IoHandler::new();
        io.add_method("say_hello", |_params: Params| {
            Ok(Value::String("hello".to_string()))
        });

        let jsonrpc_server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&"127.0.0.1:3030".parse().unwrap())
            .unwrap();

        jsonrpc_server.wait();
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
