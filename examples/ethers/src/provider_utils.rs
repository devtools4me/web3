use ethers::{
    prelude::*,
    utils
};
use eyre::Result;
use log::*;
use std::{convert::TryFrom};

const RPC_URL: &str = "http://127.0.0.1:8545/";
const WETH_ADDR: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC_ADDR: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const PRIVATE_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

pub async fn provider_calls() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    info!("block_number={block_number}");

    let gas_price: U256 = provider.get_gas_price().await?;
    info!("gas_price={}", utils::format_units(gas_price, "gwei").unwrap());

    let address_from = WETH_ADDR;
    let nonce = provider.get_transaction_count(address_from, None).await?;
    info!("nonce={}", nonce);

    let wallet: LocalWallet = PRIVATE_KEY
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::Moonbeam);

    let balance_from = provider.get_balance(address_from, None).await?;
    info!("balance_from={}", utils::format_units(balance_from, "gwei").unwrap());
    let address_to = USDC_ADDR;
    let balance_to = provider.get_balance(address_to, None).await?;
    info!("balance_to={}", utils::format_units(balance_to, "gwei").unwrap());

    Ok(())
}