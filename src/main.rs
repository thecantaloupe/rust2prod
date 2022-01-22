use rust2prod_api::startup::{run};
use rust2prod_api::configuration::get_configuration;
use rust2prod_api::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // heroku and choosing its own dang port
    let port;
    dotenv().ok();
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_e) => port = "none".to_string(),
    }
    let u16_port = port.parse::<u16>().unwrap();

    let subscriber = get_subscriber("rust2prod_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // old logger env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // No longer async, given that we don't actually try to connect!
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, u16_port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}