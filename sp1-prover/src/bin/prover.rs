use clap::Parser;
use sp1_prover::{
    aggregator::run_aggregator_client, config::ProverConfig,
    executor_service::run_executor_service, hashdb_service::run_hashdb_service,
};
use url::Url;

#[derive(Parser)]
struct Args {
    /// Url of the aggregator
    #[clap(long, default_value = "http://localhost:50081", env = "AGGREGATOR_URL")]
    aggregator_url: Url,

    /// Executor server port
    #[clap(long, env = "EXECUTOR_PORT")]
    executor_port: Option<u16>,

    /// HashDB server port
    #[clap(long, env = "HASHDB_PORT")]
    hashdb_port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // prepare config for prover
    let config = ProverConfig {
        aggregator_url: args.aggregator_url,
        executor_port: args.executor_port,
        hashdb_port: args.hashdb_port,
    };

    if config.executor_port.is_some() {
        run_executor_service(config.clone()).await;
    }

    if config.hashdb_port.is_some() {
        run_hashdb_service(config.clone()).await;
    }

    run_aggregator_client(config.clone())
        .await
        .expect("aggregator service failed");
}
