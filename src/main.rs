use blog_newsletter::configuration::get_configuration;
use blog_newsletter::email_client::EmailClient;
use blog_newsletter::startup::run;
use blog_newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("blog-newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to get configuration file!");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let authorization_token = configuration.email_client.authorization_token;
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        authorization_token,
    );
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Running on port: {}", port);
    run(listener, connection_pool, email_client)?.await
}
