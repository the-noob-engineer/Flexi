use std::process::exit;

use crate::args::Args;
use clap::Parser;
use flexi_core::config_manager::build_config_from_file;

pub fn handle_args() {
    let args = Args::parse();
    let config_file: String = match args.config_file {
        Some(value) => value,
        None => exit(1),
    };

    let config = build_config_from_file(config_file);

    match config {
        Ok(value) => {
            log::info!("{:?}", value)
        }

        Err(error) => {
            log::info!("{:?}", error);
            exit(1);
        }
    }
}
