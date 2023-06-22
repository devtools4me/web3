use ethers::{
    prelude::*,
    utils,
};
use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;
use crate::eth_abi;
use eth_abi::*;
use std::time::{Instant, Duration};

const RPC_URL: &str =
    //"https://eth-mainnet.g.alchemy.com/v2/TtK-PVc3lbV2nb7V_qUwTUALYEEBAySG";
    "http://127.0.0.1:8545/";
const WETH_ADDR: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC_ADDR: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const MY_ADDR: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
const PRIVATE_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const USDC_WHALE: &str = "0xcffad3200574698b78f32232aa9d63eabd290703";
const ADDR_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";

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

pub fn factory_pair_sync() {
    info!("factory_pair_sync");
    Runtime::new().unwrap().block_on(factory_pair()).unwrap();
}

pub async fn factory_pair() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let router_address: Address = ADDR_ROUTER.parse()?;
    let router_contract = IUniswapV2Router01::new(router_address, Arc::clone(&client));
    let weth_address = router_contract.weth().call().await?;
    info!("weth_address={}", weth_address);

    let factory_address = router_contract.factory().call().await?;
    info!("factory_address={}", factory_address);

    let factory_contract = IUniswapV2Factory::new(factory_address, Arc::clone(&client));
    let usdc_address: Address = USDC_ADDR.parse()?;
    let pair_address = factory_contract.get_pair(weth_address, usdc_address).call().await?;
    info!("pair_address={}", pair_address);

    let pair_contract = IUniswapV2Pair::new(pair_address, Arc::clone(&client));
    let symbol = pair_contract.symbol().call().await?;
    info!("symbol={}", symbol);

    let token0 = pair_contract.token_0().call().await?;
    info!("token0={}", token0);

    let token1 = pair_contract.token_1().call().await?;
    info!("token1={}", token1);

    Ok(())
}

pub fn eth_swap_sync() {
    info!("eth_swap_sync");
    Runtime::new().unwrap().block_on(eth_swap()).unwrap();
}

pub fn deadline() -> U256 {
    let start = Instant::now();
    let elapsed = start.elapsed() + Duration::from_secs(60);
    U256::from_dec_str(elapsed.as_millis().to_string().as_str()).unwrap()
}

pub async fn eth_swap() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let client = Arc::new(provider);
    let from_address: Address = WETH_ADDR.parse()?;
    let t0_contract = IERC20::new(from_address, Arc::clone(&client));
    let to_address: Address = USDC_ADDR.parse()?;
    let t1_contract = IERC20::new(to_address, Arc::clone(&client));
    let router_address: Address = ADDR_ROUTER.parse()?;
    let router_contract = IUniswapV2Router01::new(router_address, Arc::clone(&client));
    let amount_in = utils::parse_units(0.1, "ether").unwrap();
    info!("amount_in={}", amount_in);
    let amounts = router_contract.get_amounts_out(amount_in.into(), vec![from_address, to_address]).await?;
    amounts
        .iter()
        .for_each(|x| println!("{}", x));
    let my_address: Address = MY_ADDR.parse()?;
    let result = router_contract.swap_exact_tokens_for_tokens(
        amounts[0],
        amounts[1],
        vec![from_address, to_address],
        my_address,
        deadline()).await?;
    result
        .iter()
        .for_each(|x| println!("{}", x));

    Ok(())
}

pub async fn fund_erc20() -> Result<()> {
    Ok(())
}
