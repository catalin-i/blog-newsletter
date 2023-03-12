use blog_newsletter::configuration::get_configuration;
use blog_newsletter::startup::run;
use blog_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("blog-newsletter".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to get configuration file!");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Running on port: {}", port);
    run(listener, connection_pool)?.await
}
