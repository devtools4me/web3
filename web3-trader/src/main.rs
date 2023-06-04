use web3_trader::run;
use env_logger::Env;
use log::*;

mod configuration;
use configuration::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    info!("configuration={}", configuration);
    run()?.await
}
