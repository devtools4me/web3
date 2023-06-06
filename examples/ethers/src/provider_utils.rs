use ethers::{
    contract::{abigen, Abigen, ContractFactory},
    prelude::*,
    utils,
};
use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;

const RPC_URL: &str =
    //"https://eth-mainnet.g.alchemy.com/v2/TtK-PVc3lbV2nb7V_qUwTUALYEEBAySG";
    "http://127.0.0.1:8545/";
const WETH_ADDR: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC_ADDR: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const MY_ADDR: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
const PRIVATE_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const USDC_WHALE: &str = "0xcffad3200574698b78f32232aa9d63eabd290703";

pub fn provider_calls_sync() {
    info!("provider_calls_sync");
    Runtime::new().unwrap().block_on(provider_calls()).unwrap();
}

pub async fn provider_calls() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    info!("block_number={block_number}");

    let gas_price: U256 = provider.get_gas_price().await?;
    info!(
        "gas_price={}",
        utils::format_units(gas_price, "gwei").unwrap()
    );

    let address_from = WETH_ADDR;
    let nonce = provider.get_transaction_count(address_from, None).await?;
    info!("nonce={}", nonce);

    let wallet: LocalWallet = PRIVATE_KEY
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::Moonbeam);

    let balance_from = provider.get_balance(address_from, None).await?;
    info!(
        "balance_from={}",
        utils::format_units(balance_from, "gwei").unwrap()
    );
    let address_to = USDC_ADDR;
    let balance_to = provider.get_balance(address_to, None).await?;
    info!(
        "balance_to={}",
        utils::format_units(balance_to, "gwei").unwrap()
    );

    Ok(())
}

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

pub fn contract_load_sync() {
    info!("contract_load_sync");
    Runtime::new().unwrap().block_on(contract_load()).unwrap();
}

pub async fn contract_load() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let weth_address: Address = WETH_ADDR.parse()?;
    let weth_contract = IERC20::new(weth_address, Arc::clone(&client));
    let total_supply = weth_contract.total_supply().call().await?;
    info!("total_supply={} eth", utils::format_units(total_supply, "ether").unwrap());

    let balance = weth_contract.balance_of(MY_ADDR.parse()?).call().await?;
    info!("balance={} eth", utils::format_units(balance, "ether").unwrap());

    let usdc_address: Address = USDC_ADDR.parse()?;
    let usdc_contract = IERC20::new(usdc_address, Arc::clone(&client));
    let whale_balance = usdc_contract.balance_of(USDC_WHALE.parse()?).call().await?;
    info!("whale_balance={}", whale_balance);

    Ok(())
}

pub async fn fund_erc20() -> Result<()> {
    Ok(())
}
