
mod service;
mod db;
mod x;
mod xutils;
mod xmlparse;

use tonic::transport::Server;

use service::{MatsubaServer, MatsubaService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:10000".parse().unwrap();
    let inner = MatsubaService {};
    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
