use sundb_core::DatabaseConfig;
use sundb_kernel::EngineRuntime;
use sundb_network::{NetworkConfig, ServerTransport};

#[tokio::main]
async fn main() {
    let config = DatabaseConfig::default();
    let runtime = EngineRuntime::new(config.clone()).expect("runtime should initialize");
    let transport = ServerTransport::new(NetworkConfig::default(), config).expect("transport should initialize");

    println!("SUNDB server ready");
    println!("runtime status: {}", runtime.status());
    transport.start().await.expect("server should start");
}
