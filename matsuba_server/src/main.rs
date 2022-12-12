mod config;
mod db;
mod error;
mod output;
mod renderer;
mod service;
mod xmlparse;

use crate::config::Settings;
use tonic::transport::Server;

use crate::service::{MatsubaServer, MatsubaService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().format_timestamp(None).init();

    renderer::run().await;

    // manually trigger lazy static call (sorta hacky)
    let listen_address = &config::SETTINGS.server.listen_address;

    let addr = listen_address.parse().unwrap();
    // let inner = MatsubaService {xsession: session};
    let inner = MatsubaService {};

    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
