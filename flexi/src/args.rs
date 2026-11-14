use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long, default_value = "flexi.toml")]
    pub config_file: Option<String>,

    #[arg(short = 's', long, default_value = "0.0.0.0:6543")]
    pub serve: Option<String>,
}
