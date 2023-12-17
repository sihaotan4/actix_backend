use std::net::TcpListener;
use actix_backend::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let lst = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to random port");
    run(lst)?.await
}
