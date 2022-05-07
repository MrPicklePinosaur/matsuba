
pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}
use tonic::Request;
use matsubaproto::matsuba_client::MatsubaClient;
use matsubaproto::{ConvertRequest, ConvertResponse};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = MatsubaClient::connect("http://[::1]:10000").await?;

    let response = client.convert(Request::new(
        ConvertRequest {
            raw: "konnichiha".to_string(),
        }
    )).await?;
    println!("{:?}", response);

    cli::runcli()?;

    Ok(())
}
