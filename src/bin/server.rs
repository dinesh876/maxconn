use maxconn::server;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(name = "maxconn-server", version, author, about = "A maxconn server")]
struct Cli {
    #[clap(long)]
    port: Option<u16>,
    #[clap(long)]
    limit: Option<usize>
}

#[tokio::main]
async fn main()-> Result<(),Box<dyn  std::error::Error>>{
    const DEFAULT_PORT: u16 = 5001;
    const MAX_CONNECTION:usize =  1000;
    let cli  =  Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let max_connection = cli.limit.unwrap_or(MAX_CONNECTION);
    tracing_subscriber::fmt::try_init().unwrap();
    info!("starting the server");
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).await?;
    server::run(listener,max_connection,signal::ctrl_c()).await;
    Ok(())
}

