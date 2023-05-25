use web3_trader::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
