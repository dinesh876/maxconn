use tokio::sync::mpsc;
/*
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

*/

#[tokio::main]
async  fn main() -> Result<(),Box<dyn std::error::Error>> {
    let (tx,mut rx) = mpsc::channel::<u32>(100);
    let tx_clone1  = tx.clone();
    let tx_clone2 = tx.clone();
    tokio::spawn(async move {
        for i in 0..50{
            tx_clone1.send(i).await;
        }
        drop(tx_clone1)
    });
    tokio::spawn(async move {
        for i in 51..100 {
            tx_clone2.send(i).await;
        }
       // drop(tx_clone2)
    });
    drop(tx);
    while let Some(msg) = rx.recv().await {
        println!("{:?}",msg);
    }
    Ok(())
}

