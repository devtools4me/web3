use log::*;
use log4rs;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();

    info!("Starting...");

    utils::mom().await?;

    info!("Done...");

    Ok(())
}
