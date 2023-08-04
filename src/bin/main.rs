use tokio::sync::mpsc;
use maxconn::metrics::{TcpClientConnectionMetrics,ConnectionStatus};
async fn start_background_reporting(max_connection:u64,report_interval: u64) -> Result<mpsc::Sender<TcpClientConnectionMetrics>,Box<dyn std::error::Error>>{
    let capacity:usize =  (max_connection * 3) as usize;
    let (tx,rx) = mpsc::channel(capacity);
    tokio::spawn(async move{
       if let Err(e) = collect_connections_status(rx).await{
            println!("collect connection status failed with error {:?}",e)
       };
    });
    Ok(tx)

}


async fn collect_connections_status(mut status: mpsc::Receiver<TcpClientConnectionMetrics>)  -> Result<(),Box<dyn std::error::Error +'static>>{

     while let Some(message) = status.recv().await {
        println!("GOT = {}", message);
    }
    Ok(())

}

#[tokio::main]
async  fn main() -> Result<(),Box<dyn std::error::Error>> {
    let max_connection:u64 = 100;
    let report_interval:u64 = 100;
    let statusch: mpsc::Sender<TcpClientConnectionMetrics> = start_background_reporting(max_connection, report_interval).await?;
    
    for i in 0..100 {
        statusch.send(TcpClientConnectionMetrics::new(i,ConnectionStatus::ConnectionError)).await?;
    }
    Ok(())
}

