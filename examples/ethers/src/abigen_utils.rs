use ethers::{
    prelude::*
};
use eyre::Result;
use std::{path::Path};

pub fn rust_file_generation() -> Result<()> {
    let abi_source = "./abi/ERC20.json";
    let out_file = Path::new("./src/erc20.rs");
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }
    Abigen::new("ERC20", abi_source)?.generate()?.write_to_file(out_file)?;
    Ok(())
}