use maxconn::clients::client;
use tokio::signal;
use clap::Parser;
use std::env;
use maxconn::reporting;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Parser,Debug)]
#[clap(name="maxconn-client",version,author="Dinesh",about="A maxconn server")]
struct Cli{
    #[clap(long)]
    host:Option<String>,
    #[clap(long,short)]
    port: Option<u16>,
    #[clap(long,short)]
    debug: bool,
    #[clap(long,short)]
    sleep: Option<u64>,
    #[clap(long,short)]
    timeout: Option<u64>,
    #[clap(long,short)]
    connections: Option<u64>,
    #[clap(long,short)]
    interval: Option<u64>
    
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    const DEFAULT_HOST: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 =  5001;
    const DEFAULT_SLEEP_TIME: u64 = 10;
    const DEFAULT_TIMEOUT:u64 = 100;
    const MAX_CONNECTION:u64 = 100;
    const REPORT_INTERVAL:u64 = 10;
    let cli = Cli::parse();
    let host  = cli.host.unwrap_or(DEFAULT_HOST.to_string());
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let sleep = cli.sleep.unwrap_or(DEFAULT_SLEEP_TIME);
    let max_connections = cli.connections.unwrap_or(MAX_CONNECTION);
    let timeout = cli.timeout.unwrap_or(DEFAULT_TIMEOUT);
    let report_interval =  cli.interval.unwrap_or(REPORT_INTERVAL);
    let address = format!("{}:{}",host,port);
    if cli.debug {
        env::set_var("RUST_LOG","DEBUG");
    }
    let stop:Arc<AtomicBool> =  Arc::new(AtomicBool::new(false));
    tracing_subscriber::fmt::try_init().unwrap();
    let status_ch = reporting::start_background_reporting(max_connections,report_interval,stop.clone()).await?;
    client::run(address,signal::ctrl_c(),max_connections,status_ch,sleep,stop).await;
    Ok(())
}
