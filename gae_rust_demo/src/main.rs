// Include macros to be able to use `insert_routes!`.
#[macro_use]
extern crate rustful;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter};

fn say_hello(context: Context, response: Response) {
    // Get the value of the path variable `:person`, from below.
    let person = match context.variables.get("person") {
        Some(name) => name,
        None => "World".into(),
    };

    // Use the name from the path variable to say hello.
    response.send(format!("\{\"hello\":\"world\"\}"));
}

fn main() {
    env_logger::init().unwrap();

    // Build and run the server.
    let server_result = Server {
            // Turn a port number into an IPV4 host address (0.0.0.0:8080 in this case).
            host: 8080.into(),

            // Create a TreeRouter and fill it with handlers.
            handlers: insert_routes!{
            TreeRouter::new() => {
                //Handle requests for root...
                Get: say_hello,

                //...and one level below.
                //`:person` is a path variable and it will be accessible in the handler.
                ":person" => Get: say_hello
            }
        },

            // Use default values for everything else.
            ..Server::default()
        }
        .run();

    match server_result {
        Ok(_server) => {}
        Err(e) => error!("could not start server: {}", e.description()),
    }
}
