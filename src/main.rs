//! main.rs
use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
mod routes;
mod startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to establish connection to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection)?.await
}
