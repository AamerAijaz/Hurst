mod hurst;

use crate::hurst::server::Server;

fn main() {
    let server = Server::new("127.0.0.1", "8080").unwrap_or_else(|err| {
        eprintln!("Unable to initialize server: {}", err);
        panic!("Unable to initialize server");
    });

    server.serve();
}
