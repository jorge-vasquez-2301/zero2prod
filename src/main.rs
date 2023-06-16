use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    run(TcpListener::bind(format!(
        "127.0.0.1:{}",
        configuration.application_port
    ))?)?
    .await
}
