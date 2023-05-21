use ethers::{
    contract::{abigen, Abigen, ContractFactory},
    prelude::*,
    utils
};
use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};

const RPC_URL: &str = "http://127.0.0.1:8545/";
const WETH_ADDR: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC_ADDR: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const MY_ADDR: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
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

pub async fn contract_load() -> Result<()> {
    abigen!(
        IERC20,
        r#"[
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let address: Address = WETH_ADDR.parse()?;
    let contract = IERC20::new(address, client);
    let total_supply = contract.total_supply().call().await?;
    info!("total_supply={total_supply}");

    let balance = contract.balance_of(MY_ADDR.parse()?).call().await?;
    info!("balance={balance}");

    Ok(())
}