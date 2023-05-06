use std::net::TcpListener;

use sqlx::PgPool;
use env_logger::Env;

use zero2prod::configurations::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&address)
        .expect(&format!("Failed to bind to application port {}", &address));

    run(listener, connection)?.await
}
