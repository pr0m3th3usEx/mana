extern crate pump;

use std::{error::Error, path::PathBuf, time::Duration};

use clap::{arg, value_parser, ArgAction, Command};
use futures::pin_mut;
use mana_core::traits::data_collector::DataCollector;
use mana_core::value_objects::configuration::node::MAINNET_URL;
use pump::adapters::data_collector::PumpDataCollector;
use smol::stream::StreamExt;
use solana_client::rpc_client::RpcClient;

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
const CLI_BIN_NAME: &str = env!("CARGO_PKG_NAME");

async fn run(
    config_file: &PathBuf,
    wait_flag: bool,
    sandbox_flag: bool,
) -> Result<(), Box<dyn Error + 'static>> {
    // Parse configuration
    // Check token metadata
    // Check environment variables required (MANA_PRIVATE_KEY, MANA_PUBLIC_KEY)

    let rpc = RpcClient::new(MAINNET_URL);
    let data_collector = PumpDataCollector::new(&rpc, Duration::from_secs(2));
    let stream = data_collector.start().await;
    pin_mut!(stream);

    while let Some(tick) = stream.next().await {
        println!("{:?}", tick.unwrap());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut cmd = Command::new(CLI_BIN_NAME)
        .version(CLI_VERSION)
        .subcommand(
            Command::new("snipe")
                .arg(
                    arg!(-c --config <PATH_TO_FILE> "Configuration file")
                        .required(false)
                        .default_value("config.yaml")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-w --wait "Run sniper and waits until the token is released (Otherwise it tries to buy instantly")
                        .action(ArgAction::SetTrue)
                        .required(false)
                )
                .arg(
                    arg!(-s --sandbox "Execute sniping in sandbox environment (No transactions are actually made)")
                        .action(ArgAction::SetTrue)
                        .required(false)
                ),
        );
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("snipe") {
        let config_file = matches.get_one::<PathBuf>("config").unwrap();
        let wait_flag = matches.get_flag("wait");
        let sandbox_flag = matches.get_flag("sandbox");

        smol::block_on(run(config_file, wait_flag, sandbox_flag))?;
    } else {
        cmd.print_help()?;
    }
    Ok(())
}
