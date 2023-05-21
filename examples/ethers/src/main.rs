mod abigen_utils;
mod provider_utils;

use log::*;
use log4rs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();

    info!("Starting...");

    // abigen_utils::rust_file_generation();
    //provider_utils::provider_calls().await?;
    provider_utils::contract_load().await?;

    info!("Done...");

    Ok(())
}
