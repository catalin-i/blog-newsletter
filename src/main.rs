use blog_newsletter::startup::run;
use blog_newsletter::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration file!");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Running on port: {}", port);
    run(listener)?.await
}
