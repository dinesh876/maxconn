use maxconn::server;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use std::env;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(name = "maxconn-server", version, author, about = "A maxconn server")]
struct Cli {
    #[clap(long,short)]
    port: Option<u16>,
    #[clap(long,short)]
    limit: Option<usize>,
    #[clap(long,short)]
    debug: bool
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main()-> Result<(),Box<dyn  std::error::Error>>{
    const DEFAULT_PORT: u16 = 5001;
    const MAX_CONNECTION:usize =  1000;
    let cli  =  Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let max_connection = cli.limit.unwrap_or(MAX_CONNECTION);
    if cli.debug {
        env::set_var("RUST_LOG", "DEBUG");
    }
    tracing_subscriber::fmt::try_init().unwrap();
    info!("starting the server");
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).await?;
    server::run(listener,max_connection,signal::ctrl_c()).await;
    Ok(())
}

