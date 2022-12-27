use matsuba_grpc::matsuba_server::Matsuba;
pub use matsuba_grpc::matsuba_server::MatsubaServer;
use matsuba_grpc::{
    ConvertRequest, ConvertResponse, FetchRequest, FetchResponse, GetStateRequest,
    GetStateResponse, SetStateHenkanRequest, SetStateHenkanResponse, SetStateMuhenkanRequest,
    SetStateMuhenkanResponse,
};
use std::collections::HashSet;
use tonic::{Code, Request, Response, Status};

use matsuba_common::converter::Converter;

use super::{db, xmlparse};

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
    async fn convert(
        &self,
        request: Request<ConvertRequest>,
    ) -> Result<Response<ConvertResponse>, Status> {
        let request = request.get_ref();

        let mut c = Converter::new();

        let conn = db::get_connection().or(Err(Status::new(
            Code::Internal,
            "could not establish connection to database",
        )))?;

        // TODO maybe support conversion of multiple inputs at a time (batch)
        for ch in request.raw.chars() {
            c.input_char(ch);
        }
        let kana = c.accept();

        // if kana flag is passed, don't do any more conversion
        if request.kana_only {
            return Ok(Response::new(ConvertResponse {
                converted: vec![kana],
            }));
        }

        let converted = db::search(&conn, &kana)
            .or(Err(Status::new(Code::Internal, "error querying database")))?
            .iter()
            .take(request.result_count as usize)
            .map(|x| x.k_ele.clone())
            .collect::<Vec<String>>();

        Ok(Response::new(ConvertResponse { converted }))
    }

    async fn fetch(
        &self,
        request: Request<FetchRequest>,
    ) -> Result<Response<FetchResponse>, Status> {
        let request = request.get_ref();

        let mut conn = db::get_connection().or(Err(Status::new(
            Code::Internal,
            "could not establish connection to database",
        )))?;
        db::init(&conn).or(Err(Status::new(
            Code::Internal,
            "failed initializing database",
        )))?;

        let path = std::path::Path::new(&request.database_path); // TODO is this dangerous?

        // TODO stupid how we convert hashset to vec and then back to hashset
        let mut tags: HashSet<&str> = HashSet::new();
        for tag in &request.tags {
            tags.insert(tag);
        }

        xmlparse::parse_jmdict_xml(&mut conn, path, &tags)
            .or(Err(Status::new(Code::Internal, "issue parsing dict")))?;

        Ok(Response::new(FetchResponse {}))
    }

    async fn get_state(
        &self,
        _request: Request<GetStateRequest>,
    ) -> Result<Response<GetStateResponse>, Status> {
        unimplemented!();
        Ok(Response::new(GetStateResponse { henkan: true }))
    }

    async fn set_state_henkan(
        &self,
        _request: Request<SetStateHenkanRequest>,
    ) -> Result<Response<SetStateHenkanResponse>, Status> {
        unimplemented!();
        Ok(Response::new(SetStateHenkanResponse {}))
    }

    async fn set_state_muhenkan(
        &self,
        _request: Request<SetStateMuhenkanRequest>,
    ) -> Result<Response<SetStateMuhenkanResponse>, Status> {
        unimplemented!();
        Ok(Response::new(SetStateMuhenkanResponse {}))
    }
}
