use ethers::{
    prelude::*,
    utils,
};
use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc};
use tokio::runtime::Runtime;
use crate::eth_abi::{IERC20, IUniswapV2Pair, IUniswapV2Factory, IUniswapV2Router01};
use std::time::{Duration, SystemTime};

pub struct EthProps {
    pub url: String,
    pub weth_addr: String,
    pub usdc_addr: String,
    pub my_addr: String,
    pub private_key: String,
    pub usdc_whale: String,
    pub addr_router: String,
}

impl EthProps {
    pub fn provider_calls_sync(&self) {
        info!("provider_calls_sync");
        Runtime::new().unwrap().block_on(self.provider_calls()).unwrap();
    }

    pub async fn provider_calls(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(self.url.as_str())?;
        let block_number: U64 = provider.get_block_number().await?;
        info!("block_number={block_number}");

        let gas_price: U256 = provider.get_gas_price().await?;
        info!("gas_price={}", utils::format_units(gas_price, "gwei").unwrap());

        let address_from = self.weth_addr.as_str();
        let nonce = provider.get_transaction_count(address_from, None).await?;
        info!("nonce={}", nonce);

        let _wallet: LocalWallet = self.private_key
            .parse::<LocalWallet>()?
            .with_chain_id(Chain::Moonbeam);

        let balance_from = provider.get_balance(address_from, None).await?;
        info!("balance_from={}", utils::format_units(balance_from, "gwei").unwrap());

        let address_to = self.usdc_addr.as_str();
        let balance_to = provider.get_balance(address_to, None).await?;
        info!("balance_to={}", utils::format_units(balance_to, "gwei").unwrap());

        Ok(())
    }

    pub fn contract_load_sync(&self) {
        info!("contract_load_sync");
        Runtime::new().unwrap().block_on(self.contract_load()).unwrap();
    }
    
    pub async fn contract_load(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(self.url.as_str())?;
        let client = Arc::new(provider);
        let weth_address: Address = self.weth_addr.parse()?;
        let weth_contract = IERC20::new(weth_address, Arc::clone(&client));
        let total_supply = weth_contract.total_supply().call().await?;
        info!("total_supply={} eth", utils::format_units(total_supply, "ether").unwrap());
    
        let balance = weth_contract.balance_of(self.my_addr.parse()?).call().await?;
        info!("balance={} eth", utils::format_units(balance, "ether").unwrap());
    
        let usdc_address: Address = self.usdc_addr.parse()?;
        let usdc_contract = IERC20::new(usdc_address, Arc::clone(&client));
        let whale_balance = usdc_contract.balance_of(self.usdc_whale.parse()?).call().await?;
        info!("whale_balance={}", whale_balance);
    
        Ok(())
    }

    pub fn factory_pair_sync(&self) {
        info!("factory_pair_sync");
        Runtime::new().unwrap().block_on(self.factory_pair()).unwrap();
    }
    
    pub async fn factory_pair(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(self.url.as_str())?;
        let client = Arc::new(provider);
        let router_address: Address = self.addr_router.parse()?;
        let router_contract = IUniswapV2Router01::new(router_address, Arc::clone(&client));
        let weth_address = router_contract.weth().call().await?;
        info!("weth_address={}", weth_address);
    
        let factory_address = router_contract.factory().call().await?;
        info!("factory_address={}", factory_address);
    
        let factory_contract = IUniswapV2Factory::new(factory_address, Arc::clone(&client));
        let usdc_address: Address = self.usdc_addr.parse()?;
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
    
    pub fn eth_send_sync(&self) {
        info!("eth_send_sync");
        Runtime::new().unwrap().block_on(self.eth_send()).unwrap();
    }
    
    pub async fn eth_send(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(self.url.as_str())?;
        let from_address: Address = self.weth_addr.parse()?;
        let amount_in = utils::parse_units(0.1, "ether").unwrap();
        info!("amount_in={}", amount_in);
    
        let my_address: Address = self.my_addr.parse()?;
        let wallet = self.private_key
            .parse::<LocalWallet>()?
            .with_chain_id(Chain::AnvilHardhat);
        println!("Wallet: {:?}", wallet.address());
    
        let balance_before = provider.get_balance(from_address, None).await?;
        info!(
            "balance_before={}",
            utils::format_units(balance_before, "gwei").unwrap()
        );
    
        let tx = TransactionRequest::new().to(from_address).value(amount_in).from(my_address);
        let signer = SignerMiddleware::new(Provider::<Http>::try_from(self.url.as_str())?, wallet);
        let pending_tx = signer.send_transaction(tx, None).await?;
        let receipt = pending_tx
            .await?
            .ok_or_else(|| eyre::format_err!("tx not included"))?;
        let tx = provider.get_transaction(receipt.transaction_hash).await?;
        println!("tx: {:?}", tx);
        // println!("Sent transaction: {}\n", serde_json::to_string(&tx)?);
        // println!("Receipt: {}\n", serde_json::to_string(&receipt)?);
    
        let balance_after = provider.get_balance(from_address, None).await?;
        info!(
            "balance_after={}",
            utils::format_units(balance_after, "gwei").unwrap()
        );
    
        Ok(())
    }
    
    pub fn eth_swap_sync(&self) {
        info!("eth_swap_sync");
        Runtime::new().unwrap().block_on(self.eth_swap()).unwrap();
    }
    
    pub async fn eth_swap(&self) -> Result<()> {
        let provider = Provider::<Http>::try_from(self.url.as_str())?;
        let client = Arc::new(Provider::<Http>::try_from(self.url.as_str())?);
        let from_address: Address = self.weth_addr.parse()?;
        let t0_contract = IERC20::new(from_address, Arc::clone(&client));
        let to_address: Address = self.usdc_addr.parse()?;
        let t1_contract = IERC20::new(to_address, Arc::clone(&client));
        let router_address: Address = self.addr_router.parse()?;
        let router_contract = IUniswapV2Router01::new(router_address, Arc::clone(&client));
        let amount_in = utils::parse_units(0.1, "ether").unwrap();
        info!("amount_in={}", amount_in);
        let amounts = router_contract.get_amounts_out(amount_in.into(), vec![from_address, to_address]).await?;
        amounts
            .iter()
            .for_each(|x| println!("{}", x));
        let my_address: Address = self.my_addr.parse()?;
    
        let wallet = self.private_key
            .parse::<LocalWallet>()?
            .with_chain_id(Chain::AnvilHardhat);
        println!("Wallet: {:?}", wallet.address());
    
        let balance_before = provider.get_balance(from_address, None).await?;
        info!(
            "balance_before={}",
            utils::format_units(balance_before, "gwei").unwrap()
        );
    
        let tx = TransactionRequest::new().to(from_address).value(amount_in).from(my_address);
        let signer = SignerMiddleware::new(Provider::<Http>::try_from(self.url.as_str())?, wallet);
        let pending_tx = signer.send_transaction(tx, None).await?;
        let receipt = pending_tx
            .await?
            .ok_or_else(|| eyre::format_err!("tx not included"))?;
        let tx = client.get_transaction(receipt.transaction_hash).await?;
        println!("tx: {:?}", tx);
        // println!("Sent transaction: {}\n", serde_json::to_string(&tx)?);
        // println!("Receipt: {}\n", serde_json::to_string(&receipt)?);
    
        let balance_after = provider.get_balance(from_address, None).await?;
        info!(
            "balance_after={}",
            utils::format_units(balance_after, "gwei").unwrap()
        );
    
        let result = router_contract.swap_exact_tokens_for_tokens(
            amounts[0],
            amounts[1],
            vec![from_address, to_address],
            my_address,
            deadline(60)).await?;
        result
            .iter()
            .for_each(|x| println!("{}", x));
    
        Ok(())
    }
    
    pub async fn fund_erc20() -> Result<()> {
        Ok(())
    }
}
    
pub fn now() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}

pub fn deadline(deadline: u64) -> U256 {
    U256::from(deadline + now().as_secs())
}