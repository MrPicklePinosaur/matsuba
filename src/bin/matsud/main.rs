
pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use matsubaproto::matsuba_server::{Matsuba, MatsubaServer};
use matsubaproto::{ConvertRequest, ConvertResponse};

#[derive(Debug)]
struct MatsubaService;

#[tonic::async_trait]
impl Matsuba for MatsubaService {

    async fn convert(&self, request: Request<ConvertRequest>) -> Result<Response<ConvertResponse>,Status> {
        Ok(Response::new(ConvertResponse{converted: "こんにちは世界".to_string() }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:10000".parse().unwrap();
    let inner = MatsubaService {};
    let svc = MatsubaServer::new(inner);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
