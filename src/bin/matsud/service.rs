
pub mod matsubaproto {
    tonic::include_proto!("matsubaproto");
}

use matsubaproto::matsuba_server::Matsuba;
use matsubaproto::{
    ConvertRequest, ConvertResponse,
    GetStateRequest, GetStateResponse,
    FetchRequest, FetchResponse
};
pub use matsubaproto::matsuba_server::MatsubaServer;
use tonic::{Request, Response, Status, Code};
use std::collections::HashSet;

use super::{
    db::SearchHeuristic,
    x,
    xmlparse,
    converter::{Converter, build_dfa},
};

pub struct MatsubaService {
    // pub xsession: x::XSession
}

impl MatsubaService {

    /*
    pub fn new() {

    }
    */

}

#[tonic::async_trait]
impl Matsuba for MatsubaService {

    async fn convert(&self, request: Request<ConvertRequest>) -> Result<Response<ConvertResponse>,Status> {
        let request = request.get_ref();

        let mut c = Converter::new();

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

        let converted = db::search(&conn, &kana, SearchHeuristic::None)
            .or(Err(Status::new(Code::Internal, "error querying database")))?
            .iter()
            .take(request.result_count as usize)
            .map(|x| x.k_ele.clone())
            .collect::<Vec<String>>();

        Ok(Response::new(ConvertResponse{converted: converted}))
    }

    async fn get_state(&self, request: Request<GetStateRequest>) -> Result<Response<GetStateResponse>,Status> {
        Ok(Response::new(GetStateResponse{henkan: true}))
    }

    async fn fetch(&self, request: Request<FetchRequest>) -> Result<Response<FetchResponse>,Status> {

        let request = request.get_ref();

        let mut conn = db::get_connection()
            .or(Err(Status::new(Code::Internal, "could not establish connection to database")))?;
        db::init(&conn)
            .or(Err(Status::new(Code::Internal, "failed initializing database")))?;

        let path = std::path::Path::new(&request.database_path); // TODO is this dangerous?

        // TODO stupid how we convert hashset to vec and then back to hashset
        let mut tags: HashSet<&str> = HashSet::new();
        for tag in &request.tags {
            tags.insert(tag);
        }

        xmlparse::parse_jmdict_xml(&mut conn, path, &tags)
            .or(Err(Status::new(Code::Internal, "issue parsing dict")))?;

        Ok(Response::new(FetchResponse{}))
    }

}
