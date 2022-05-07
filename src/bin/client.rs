
pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}
use matsubaproto::matsuba_client::MatsubaClient;
use matsubaproto::{ConvertRequest, ConvertResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MatsubaClient::connect("http://[::1]:10000").await?;

    Ok(())
}
