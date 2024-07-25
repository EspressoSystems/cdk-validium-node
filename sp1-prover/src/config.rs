use url::Url;

/// Configuration/Parameters used for SP1 prover
#[derive(Debug, Clone)]
pub struct ProverConfig {
    /// URL of the aggregator.
    pub aggregator_url: Url,
    /// The port of the executor service.
    pub executor_port: Option<u16>,
    /// The port of the hashdb service
    pub hashdb_port: Option<u16>,
}
