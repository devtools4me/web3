use env_logger;
use log::*;

mod api;
mod configuration;
mod service;
mod model;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    info!("configuration={}", configuration);
    api::run_with_config(configuration)?.await
}
