
pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}

use matsubaproto::matsuba_server::Matsuba;
use matsubaproto::{ConvertRequest, ConvertResponse};
pub use matsubaproto::matsuba_server::MatsubaServer;
use tonic::{Request, Response, Status, Code};

use matsuba::converter::{Converter, build_dfa};
use super::db;

#[derive(Debug)]
pub struct MatsubaService;

#[tonic::async_trait]
impl Matsuba for MatsubaService {

    async fn convert(&self, request: Request<ConvertRequest>) -> Result<Response<ConvertResponse>,Status> {
        let request = request.get_ref();

        let dfa = build_dfa();
        let mut c = Converter::new(&dfa);

        let conn = db::get_connection()
            .or(Err(Status::new(Code::Internal, "could not establish connection to database")))?;
        
        // TODO maybe support conversion of multiple inputs at a time (batch)
        for ch in request.raw.chars() {
            c.input_char(ch);
        }
        let kana = c.accept();

        // if kana flag is passed, don't do any more conversion
        if request.kana_only {
            return Ok(Response::new(ConvertResponse{converted: vec!(kana)}));
        }

        let converted = db::search(&conn, &kana)
            .or(Err(Status::new(Code::Internal, "error querying database")))?
            .iter()
            .take(request.result_count as usize)
            .map(|x| x.k_ele.clone())
            .collect::<Vec<String>>();

        Ok(Response::new(ConvertResponse{converted: converted}))
    }

}

