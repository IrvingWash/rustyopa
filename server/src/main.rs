use server::Server;

mod http;
mod server;

fn main() {
    let s = Server::new("127.0.0.1:8080".to_string());

    s.run();
}
