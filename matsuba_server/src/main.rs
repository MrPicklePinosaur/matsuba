mod config;
mod conversion;
mod converter;
mod db;
mod error;
mod service;
mod xmlparse;

use tonic::transport::Server;

use crate::service::{MatsubaServer, MatsubaService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().format_timestamp(None).init();

    let addr = "[::1]:10000".parse().unwrap();
    // let inner = MatsubaService {xsession: session};
    let inner = MatsubaService {};

    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
