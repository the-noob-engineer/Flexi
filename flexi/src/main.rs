use clap::Parser;
use flexi::server::run_server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    serve: bool,

    #[arg(short, long, default_value = "7259")]
    port: u16,

    #[arg(short, long, default_value = "flexi.toml")]
    config_file: Option<String>,

    // datbase
    #[arg(short, long, default_value = "localhost")]
    db_host: Option<String>,

    #[arg(short, long, default_value = "8080")]
    db_port: Option<u16>,

    #[arg(short, long, default_value = "main")]
    db_namespace: Option<String>,

    #[arg(short, long, default_value = "")]
    db_user: Option<String>,

    #[arg(short, long, default_value = "")]
    db_password: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let args = Args::parse();

    if args.serve {
        run_server(Some(args.port)).await;
    }
}
