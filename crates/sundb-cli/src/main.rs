use sundb_core::DatabaseConfig;
use sundb_kernel::EngineRuntime;
use sundb_query::QueryEngine;

fn main() {
    let config = DatabaseConfig::default();
    let runtime = EngineRuntime::new(config.clone()).expect("runtime should initialize");
    let query = QueryEngine::new(config).expect("query engine should initialize");

    println!("SUNDB CLI ready");
    println!("runtime status: {}", runtime.status());
    println!("query result: {}", query.execute("SELECT 1").unwrap());
}
