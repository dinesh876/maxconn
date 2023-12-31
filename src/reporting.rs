use tokio::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicBool;
use crate::{TcpClientConnectionMetrics,GroupOfConnections,is_ok};
use std::sync::atomic::Ordering::Relaxed;
use tracing::{info,error,debug};


pub async fn start_background_reporting(max_connection:u64,report_interval: u64,stop_sig:Arc<AtomicBool>) -> Result<mpsc::Sender<TcpClientConnectionMetrics>,Box<dyn std::error::Error>>{
    let capacity:usize =  (max_connection * 3) as usize;
    let (tx,rx) = mpsc::channel::<TcpClientConnectionMetrics>(capacity);
    let connection_status_tracker: Arc<Mutex<GroupOfConnections>> = Arc::new(Mutex::new(GroupOfConnections::new(max_connection)));
    let connection_status_tracker_clone  = connection_status_tracker.clone();
    tokio::spawn(async move{
       if let Err(e) = collect_connections_status(connection_status_tracker_clone,rx).await{
            error!("collect connection status function failed with error {:?}",e)
       };
    });
    tokio::spawn(async move {
        if let Err(e) = report_connection_status(connection_status_tracker,report_interval,stop_sig).await{
            error!("report connection function failed with error {:?}",e)
        };
    });
    Ok(tx)

}


async fn collect_connections_status(connection_registry: Arc<Mutex<GroupOfConnections>>,mut status: mpsc::Receiver<TcpClientConnectionMetrics>)  -> Result<(),Box<dyn std::error::Error +'static>>{

     while let Some(message) = status.recv().await {
        {
            let mut connection_registry_locked = connection_registry.lock().unwrap();
            match connection_registry_locked.connections.get(message.id as usize) {
                Some(_) => {
                    if is_ok(&message) {
                        connection_registry_locked.max_concurrent_established +=1;
                    }
                    connection_registry_locked.connections[message.id as usize] = message
                },
                None => {
                     connection_registry_locked.connections.push(message)
                }
            };
        }
    }
    debug!("report receiver channel closed..");
    Ok(())

}


async fn report_connection_status(connection_registry:Arc<Mutex<GroupOfConnections>>,interval_between_update:u64,stop_sig:Arc<AtomicBool>) -> Result<(),Box<dyn std::error::Error>>{

    while !stop_sig.clone().load(Relaxed){
        
        if interval_between_update ==  0 {
            break;
        }
        
         {
            println!("{}",connection_registry.lock().unwrap())
         };
        
         thread::sleep(Duration::from_millis(interval_between_update));

     }
     println!("Total Connection Established:{:?}",connection_registry.lock().unwrap().max_concurrent_established);
    Ok(())
}

