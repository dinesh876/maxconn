use maxconn::clients::client;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let address = "10.213.175.1:5002".to_string();
    let max_connections:u64 = 5;
    let delay:u64 = 1;

    client::run(address,signal::ctrl_c(),max_connections,delay).await;
    Ok(())
}
