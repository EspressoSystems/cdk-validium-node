use std::{collections::HashMap, time::SystemTime};

use aggregator::{
    aggregator_message::Request, aggregator_service_client::AggregatorServiceClient,
    get_proof_response::Proof, prover_message::Response, FinalProof, GetStatusResponse,
    ProverMessage, PublicInputs, PublicInputsExtended,
};
use anyhow::Ok;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::config::ProverConfig;

pub mod aggregator {
    tonic::include_proto!("aggregator.v1");
}

static MOCKED_DATA: &str = include_str!("data/mocked_data.json");

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum RequestType {
    None,
    GenBatchProof,
    GenAggregatedProof,
    GenFinalProof,
    ProcessBatch,
    Execute,
}

pub async fn run_aggregator_client(config: ProverConfig) -> anyhow::Result<()> {
    println!(
        "Establishing connection to aggregator on: {}",
        config.aggregator_url
    );
    // create aggregator client
    let mut aggregator_client =
        AggregatorServiceClient::connect(config.aggregator_url.to_string()).await?;

    let (tx, rx) = mpsc::unbounded_channel();

    let rx_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let response_stream = aggregator_client.channel(rx_stream).await?;
    let mut response = response_stream.into_inner();

    let mut request_type = RequestType::None;
    let mut last_generated_uuid = String::new();

    while let Some(received) = response.next().await {
        if let Result::Ok(agg_msg) = received {
            // we return the same ID we got in the aggregator message
            let response_id = agg_msg.id;

            match agg_msg.request {
                Some(v) => match v {
                    Request::GetStatusRequest(_) => {
                        println!("Received GetStatusRequest");
                        let msg = ProverMessage {
                            id: response_id,
                            response: Some(Response::GetStatusResponse(gen_response())),
                        };
                        _ = tx.send(msg);
                    }

                    Request::GenBatchProofRequest(req) => {
                        println!("Received request: {:?}", req);
                        request_type = RequestType::GenBatchProof;
                        let uuid = get_uuid();
                        last_generated_uuid.clone_from(&uuid);
                        let response = aggregator::GenBatchProofResponse {
                            id: uuid.clone(),
                            result: aggregator::Result::Ok.into(),
                        };
                        let msg = ProverMessage {
                            id: response_id,
                            response: Some(Response::GenBatchProofResponse(response)),
                        };
                        _ = tx.send(msg);
                    }
                    Request::GenAggregatedProofRequest(req) => {
                        println!("Received request: {:?}", req);
                        request_type = RequestType::GenAggregatedProof;
                        let uuid = get_uuid();
                        last_generated_uuid.clone_from(&uuid);
                        let response = aggregator::GenAggregatedProofResponse {
                            id: uuid.clone(),
                            result: aggregator::Result::Ok.into(),
                        };
                        let msg = ProverMessage {
                            id: response_id,
                            response: Some(Response::GenAggregatedProofResponse(response)),
                        };
                        _ = tx.send(msg);
                    }
                    Request::GenFinalProofRequest(req) => {
                        println!("Received request: {:?}", req);
                        request_type = RequestType::GenFinalProof;
                        let uuid = get_uuid();
                        last_generated_uuid.clone_from(&uuid);
                        let response = aggregator::GenFinalProofResponse {
                            id: uuid.clone(),
                            result: aggregator::Result::Ok.into(),
                        };
                        let msg = ProverMessage {
                            id: response_id,
                            response: Some(Response::GenFinalProofResponse(response)),
                        };
                        _ = tx.send(msg);
                    }

                    Request::GetProofRequest(req) => {
                        println!("Received request: {:?}", req);
                        let req_id = req.id;
                        if req_id == last_generated_uuid {
                            let result: i32 =
                                aggregator::get_proof_response::Result::CompletedOk.into();
                            let response = aggregator::GetProofResponse {
                                id: req_id,
                                result_string: "completed".to_string(),
                                proof: get_proof(request_type.clone()),
                                result,
                            };
                            let msg = ProverMessage {
                                id: response_id,
                                response: Some(Response::GetProofResponse(response)),
                            };
                            _ = tx.send(msg);
                        }
                    }
                    _ => {
                        println!("Received an invalid message type: {:?}", v);
                    }
                },
                None => {
                    println!("Stream to aggregator completed");
                }
            }
        }
    }
    Ok(())
}

// emulate original (zkevm-prover) aggregator client mock:
// https://github.com/0xPolygonHermez/zkevm-prover/blob/d23715e37e1ceb048babd5a258147bb8f66ccc5e/test/service/aggregator/aggregator_client_mock.cpp#L34
fn get_proof(request_type: RequestType) -> Option<Proof> {
    // deserialize mock data
    let mock_data: Value = serde_json::from_str(MOCKED_DATA).expect("failed to parse json content");
    let mock_string = mock_data["bytes"].as_str().unwrap();
    let mock_bytes = hex::decode(mock_string).unwrap();
    let mock_root = hex::decode(mock_data["root"].as_str().unwrap()).unwrap();
    let mock_proof = mock_data["proof"].as_str().unwrap();
    let mock_new_acc_input_hash =
        hex::decode(mock_data["new_acc_input_hash"].as_str().unwrap()).unwrap();
    let mock_new_local_exit_root =
        hex::decode(mock_data["new_local_exit_root"].as_str().unwrap()).unwrap();
    let mock_recursive_proof_1 = mock_data["recursive_proof_1"].as_str().unwrap();
    let mock_recurisve_proof_2 = mock_data["recursive_proof_2"].as_str().unwrap();

    match request_type {
        RequestType::GenFinalProof => {
            let public_inputs = PublicInputs {
                old_state_root: mock_root.clone(),
                old_acc_input_hash: mock_root.clone(),
                old_batch_num: 1,
                chain_id: 1000,
                fork_id: 9,
                batch_l2_data: mock_bytes.clone(),
                l1_info_root: mock_bytes.clone(),
                timestamp_limit: 1_000_000,
                sequencer_addr: mock_string.to_string(),
                forced_blockhash_l1: mock_bytes.clone(),
                aggregator_addr: mock_string.to_string(),
                l1_info_tree_data: HashMap::new(),
            };
            let public_inputs_extended = PublicInputsExtended {
                public_inputs: Some(public_inputs),
                new_state_root: mock_root.clone(),
                new_acc_input_hash: mock_new_acc_input_hash,
                new_local_exit_root: mock_new_local_exit_root,
                new_batch_num: 2,
            };
            Some(Proof::FinalProof(FinalProof {
                proof: mock_proof.to_string(),
                public: Some(public_inputs_extended),
            }))
        }
        RequestType::GenBatchProof => {
            Some(Proof::RecursiveProof(mock_recursive_proof_1.to_string()))
        }
        RequestType::GenAggregatedProof => {
            Some(Proof::RecursiveProof(mock_recurisve_proof_2.to_string()))
        }
        _ => {
            println!(
                "Received GetProof with invalid request type: {:?}",
                request_type
            );
            None
        }
    }
}

fn gen_response() -> aggregator::GetStatusResponse {
    GetStatusResponse {
        status: aggregator::get_status_response::Status::Idle.into(),
        last_computed_request_id: "".to_string(),
        last_computed_end_time: get_current_time(),
        current_computing_request_id: "".to_string(),
        current_computing_start_time: get_current_time(),
        version_proto: "v0_0_1".to_string(),
        version_server: "0.0.1".to_string(),
        pending_request_queue_ids: vec![get_uuid(), get_uuid(), get_uuid()],
        prover_name: "sp1_test_prover".to_string(),
        prover_id: get_uuid(),
        fork_id: 9, // PROVER_FORK_ID

        // below constants should be read from proc or sysctl
        number_of_cores: 10,
        total_memory: 1_000_000_000,
        free_memory: 1_000_000,
    }
}

// utils to emulate functionality aggregator client mock
fn get_uuid() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(37)
        .map(char::from)
        .collect()
}

fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
