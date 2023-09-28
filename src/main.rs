use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    println!(
        "🚀 Listening on http://127.0.0.1:{}",
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}
