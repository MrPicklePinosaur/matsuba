mod converter;
mod db;
mod service;
mod x;
mod xmlparse;
mod xutils;

use matsuba::error::BoxResult;
use tonic::transport::Server;
use x11rb::connection::Connection;

use crate::service::{MatsubaServer, MatsubaService};

#[tokio::main]
async fn main() -> BoxResult<()> {
    env_logger::builder().format_timestamp(None).init();

    let mut session = x::XSession::new()?;
    session.run()?;

    let addr = "[::1]:10000".parse().unwrap();
    // let inner = MatsubaService {xsession: session};
    let inner = MatsubaService {};

    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
