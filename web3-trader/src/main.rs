use env_logger::Env;
use log::*;

mod api;
mod configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    info!("configuration={}", configuration);
    api::run()?.await
}
