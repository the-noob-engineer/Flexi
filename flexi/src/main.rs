use clap::Parser;
use flexi::server::run_server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    server: bool,

    #[arg(short, long, default_value = "7259")]
    port: u16,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let args = Args::parse();

    if args.server {
        run_server(Some(args.port)).await;
    }
}
