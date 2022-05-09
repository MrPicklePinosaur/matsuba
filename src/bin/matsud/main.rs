
mod service;
mod db;
mod converter;
mod x;
mod xutils;
mod xmlparse;

use x11rb::connection::Connection;
use tonic::transport::Server;
use matsuba::error::BoxResult;

use service::{MatsubaServer, MatsubaService};

#[tokio::main]
async fn main() -> BoxResult<()> {

    let mut session = x::XSession::new()?;
    session.configure_root()?;

    let addr = "[::1]:10000".parse().unwrap();
    let inner = MatsubaService {xsession: session};
    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
