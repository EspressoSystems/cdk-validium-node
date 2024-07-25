use hashdb_service::{
    hash_db_service_server::{HashDbService, HashDbServiceServer},
    CancelBatchRequest, CancelBatchResponse, ConsolidateStateRequest, ConsolidateStateResponse,
    Fea, FinishBlockRequest, FinishTxRequest, FlushRequest, FlushResponse, GetFlushDataRequest,
    GetFlushDataResponse, GetFlushStatusResponse, GetLatestStateRootResponse, GetProgramRequest,
    GetProgramResponse, GetRequest, GetResponse, LoadDbRequest, LoadProgramDbRequest, PurgeRequest,
    PurgeResponse, ReadTreeRequest, ReadTreeResponse, ResetDbResponse, SetProgramRequest,
    SetProgramResponse, SetRequest, SetResponse, StartBlockRequest,
};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tonic::transport::Server;

use crate::config::ProverConfig;

pub mod hashdb_service {
    tonic::include_proto!("hashdb.v1"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct SP1HashDbService {}

#[tonic::async_trait]
impl HashDbService for SP1HashDbService {
    async fn get_latest_state_root(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<GetLatestStateRootResponse>, tonic::Status> {
        todo!();
    }
    async fn set(
        &self,
        request: tonic::Request<SetRequest>,
    ) -> std::result::Result<tonic::Response<SetResponse>, tonic::Status> {
        println!("Received SetRequest: {:?}", request);
        let request = request.into_inner();
        let response = generate_set_response(request);
        Ok(tonic::Response::new(response))
    }
    async fn get(
        &self,
        _request: tonic::Request<GetRequest>,
    ) -> std::result::Result<tonic::Response<GetResponse>, tonic::Status> {
        todo!();
    }
    async fn set_program(
        &self,
        _request: tonic::Request<SetProgramRequest>,
    ) -> std::result::Result<tonic::Response<SetProgramResponse>, tonic::Status> {
        todo!();
    }
    async fn get_program(
        &self,
        _request: tonic::Request<GetProgramRequest>,
    ) -> std::result::Result<tonic::Response<GetProgramResponse>, tonic::Status> {
        todo!();
    }
    async fn load_db(
        &self,
        _request: tonic::Request<LoadDbRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!();
    }
    async fn load_program_db(
        &self,
        _request: tonic::Request<LoadProgramDbRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!();
    }
    async fn finish_tx(
        &self,
        _request: tonic::Request<FinishTxRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!();
    }
    async fn start_block(
        &self,
        _request: tonic::Request<StartBlockRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
    async fn finish_block(
        &self,
        _request: tonic::Request<FinishBlockRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!();
    }
    async fn flush(
        &self,
        _request: tonic::Request<FlushRequest>,
    ) -> std::result::Result<tonic::Response<FlushResponse>, tonic::Status> {
        todo!();
    }
    async fn get_flush_status(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<GetFlushStatusResponse>, tonic::Status> {
        todo!();
    }
    async fn get_flush_data(
        &self,
        _request: tonic::Request<GetFlushDataRequest>,
    ) -> std::result::Result<tonic::Response<GetFlushDataResponse>, tonic::Status> {
        todo!();
    }
    async fn consolidate_state(
        &self,
        _request: tonic::Request<ConsolidateStateRequest>,
    ) -> std::result::Result<tonic::Response<ConsolidateStateResponse>, tonic::Status> {
        todo!();
    }
    async fn purge(
        &self,
        _request: tonic::Request<PurgeRequest>,
    ) -> std::result::Result<tonic::Response<PurgeResponse>, tonic::Status> {
        todo!();
    }
    async fn read_tree(
        &self,
        _request: tonic::Request<ReadTreeRequest>,
    ) -> std::result::Result<tonic::Response<ReadTreeResponse>, tonic::Status> {
        todo!();
    }
    async fn cancel_batch(
        &self,
        _request: tonic::Request<CancelBatchRequest>,
    ) -> std::result::Result<tonic::Response<CancelBatchResponse>, tonic::Status> {
        todo!();
    }
    async fn reset_db(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<ResetDbResponse>, tonic::Status> {
        todo!();
    }
}

pub async fn run_hashdb_service(config: ProverConfig) -> tokio::task::JoinHandle<()> {
    let service = SP1HashDbService::default();

    let server = HashDbServiceServer::new(service);
    let socket = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.hashdb_port.unwrap(),
    );

    let server_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(server)
            .serve(socket)
            .await
            .unwrap();
    });
    println!("Starting hashdb service on: {}", socket);

    server_handle
}

fn generate_set_response(_request: SetRequest) -> SetResponse {
    SetResponse {
        old_root: None,
        new_root: Some(Fea {
            fe0: 0,
            fe1: 0,
            fe2: 0,
            fe3: 0,
        }),
        key: None,
        siblings: HashMap::new(),
        ins_key: None,
        ins_value: String::new(),
        is_old0: false,
        old_value: String::new(),
        new_value: String::new(),
        mode: String::new(),
        proof_hash_counter: 0,
        db_read_log: HashMap::new(),
        result: None,
        sibling_left_child: None,
        sibling_right_child: None,
    }
}
