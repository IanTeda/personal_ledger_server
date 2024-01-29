use personal_ledger_server::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration file
    // TODO: Update get configuration fail code
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.api_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}

// References
// https://github.com/actix/examples/tree/master/basics/nested-routing
// https://masteringbackend.com/posts/actix-web-the-ultimate-guide
