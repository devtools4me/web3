mod abigen_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    abigen_utils::rust_file_generation();
    Ok(())
}
