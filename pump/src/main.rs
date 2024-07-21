extern crate pump;

use std::{error::Error, path::PathBuf};

use clap::{arg, value_parser, ArgAction, Command};

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
const CLI_BIN_NAME: &str = env!("CARGO_PKG_NAME");

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
                ),
        );
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("snipe") {
        let config_file = matches.get_one::<PathBuf>("config").unwrap();
        let should_wait = matches.get_flag("wait");

        println!("config_file: {:?}", config_file);
        println!("should_wait: {}", should_wait);
    }

    cmd.print_help()?;

    Ok(())
}
