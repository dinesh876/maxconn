use tokio::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::{TcpClientConnectionMetrics,GroupOfConnections};
async fn start_background_reporting(max_connection:u64,report_interval: u64) -> Result<mpsc::Sender<TcpClientConnectionMetrics>,Box<dyn std::error::Error>>{
    let capacity:usize =  (max_connection * 3) as usize;
    let (tx,rx) = mpsc::channel(capacity);
    let connection_status_tracker: GroupOfConnections = Arc::new(GroupOfConnections::new(max_connection));
    let connection_status_tracker_clone  = connection_status_tracker;
    tokio::spawn(async move{
       if let Err(e) = collect_connections_status(rx).await{
            println!("collect connection status failed with error {:?}",e)
       };
    });
    tokio::spwan(async move {
        if let Err(e) = report_connection_status(connection_status_tracker_clone,report_interval).await{
            println!("report connection status module failed...")
        };
    });
    Ok((tx,connection_status_tracker))

}


async fn collect_connections_status(mut status: mpsc::Receiver<TcpClientConnectionMetrics>)  -> Result<(),Box<dyn std::error::Error +'static>>{

     while let Some(message) = status.recv().await {
        println!("GOT = {}", message);
    }
    Ok(())

}

async fn report_connection_status(gc:GroupOfConnections,interval_between_update:u64) -> Result<(),Box<dyn std::error::Error>>{

    loop{
        println!(gc)
        if interval_between_update ==  0 {
            break;
        }
        thread::sleep(Duration::millis(interval_between_update));

    }
    Ok(())
}

/*
#[tokio::main]
async  fn main() -> Result<(),Box<dyn std::error::Error>> {
    let max_connection:u64 = 100;
    let report_interval:u64 = 100;
    let statusch: mpsc::Sender<TcpClientConnectionMetrics> = start_background_reporting(max_connection, report_interval).await?;

    for i in 0..100 {
        statusch.send(TcpClientConnectionMetrics::new(i,ConnectionStatus::ConnectionError)).await?;
    }
    Ok(())
}*/
