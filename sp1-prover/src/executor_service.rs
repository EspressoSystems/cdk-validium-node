use executor_service::{
    executor_service_server::{ExecutorService, ExecutorServiceServer},
    GetFlushStatusResponse, ProcessBatchRequest, ProcessBatchRequestV2, ProcessBatchResponse,
    ProcessBatchResponseV2,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;

use crate::config::ProverConfig;

pub mod executor_service {
    tonic::include_proto!("executor.v1"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct SP1ExecutorService {}

#[tonic::async_trait]
impl ExecutorService for SP1ExecutorService {
    async fn process_batch(
        &self,
        _request: tonic::Request<ProcessBatchRequest>,
    ) -> std::result::Result<tonic::Response<ProcessBatchResponse>, tonic::Status> {
        todo!();
    }
    async fn process_batch_v2(
        &self,
        _request: tonic::Request<ProcessBatchRequestV2>,
    ) -> std::result::Result<tonic::Response<ProcessBatchResponseV2>, tonic::Status> {
        todo!();
    }
    async fn get_flush_status(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<GetFlushStatusResponse>, tonic::Status> {
        todo!();
    }
}

pub async fn run_executor_service(config: ProverConfig) -> tokio::task::JoinHandle<()> {
    let service = SP1ExecutorService::default();

    let server = ExecutorServiceServer::new(service);
    let socket = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.executor_port.unwrap(),
    );

    let server_handle = tokio::spawn(async move {
        Server::builder()
            .add_service(server)
            .serve(socket)
            .await
            .unwrap();
    });
    println!("Starting executor on: {}", socket);

    server_handle
}
