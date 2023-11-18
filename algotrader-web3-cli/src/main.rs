extern crate clap;

use algotrader_common::utils::env_utils::*;
use algotrader_web3::eth::types::*;

use dialoguer::{theme::ColorfulTheme, Select};

use log::*;
use log4rs;

fn main() {
    log4rs::init_file("../conf/log4rs.yaml", Default::default()).unwrap();

    info!("Starting...");

    let props = EthProps {
        url: get_eth_url(),
        weth_addr: get_weth_addr(),
        usdc_addr: get_usdc_addr(),
        my_addr: get_wallet_addr(),
        private_key: get_wallet_priv_key(),
        usdc_whale: get_usdc_whale(),
        addr_router: get_addr_router()
    };

    run(props);

    info!("Done...");
}

pub fn run(props: EthProps) {
    loop {
        let choices = [
            "Contract Load",
            "Provider Calls",
            "Factory Pair",
            "Eth Send",
            "Eth Swap",
            "Exit",
        ];
        let index = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue,
        };

        info!("index={}", index);

        match index {
            0 => props.contract_load_sync(),
            1 => props.provider_calls_sync(),
            2 => props.factory_pair_sync(),
            3 => props.eth_send_sync(),
            4 => props.eth_swap_sync(),
            5 => break,
            _ => continue,
        };
    }
}
