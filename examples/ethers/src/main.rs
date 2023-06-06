#[macro_use]
extern crate clap;

mod provider_utils;

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
            "Contract Load",
            "Provider Calls",
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
            0 => provider_utils::contract_load_sync(),
            1 => provider_utils::provider_calls_sync(),
            2 => break,
            _ => continue,
        };
    }
}
