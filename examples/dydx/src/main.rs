mod dydx_utils;
mod constants;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, Select};

use log::*;
use log4rs;

fn main() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();

    info!("Starting...");

    run();

    info!("Done...");
}

pub fn run() {
    loop {
        let choices = [
            "Get Markets",
            "Get Account",
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
            0 => dydx_utils::dydx_get_markets_sync(),
            1 => dydx_utils::dydx_get_account_sync(),
            2 => break,
            _ => continue,
        };
    }
}
