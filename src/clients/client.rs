use crate::{Shutdown,Connection};
use std::time::{Instant,Duration};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::time::timeout;
use std::future::Future;
use std::thread;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::{TcpClientConnectionMetrics,ConnectionStatus};
use tracing::{info,debug};

pub struct Client{
    remote_address:String,
    sh: Shutdown,
}

async fn report_tcp_client_connection_metrics(connection_details: TcpClientConnectionMetrics,status_ch:mpsc::Sender<TcpClientConnectionMetrics>) {
    let _ = status_ch.send(connection_details).await;
    info!(%connection_details,"Tcp Client Connection metrics");
    drop(status_ch);
}

pub async fn run(address: String,shutdown: impl Future + Send + 'static,max_connections: u64,status_ch:mpsc::Sender<TcpClientConnectionMetrics>,delay: u64,stop:Arc<AtomicBool>){
    let stop_clone =  stop.clone();
    let (notify_shutdown,_) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);
    tokio::spawn(async move {
        tokio::select!{
            _ = shutdown => {
                println!("shutting down...")
            } 
        }
        stop_clone.store(true,Relaxed);
    });

    for runner in 0..max_connections {
        let id = runner;

        if stop.load(Relaxed) {
            println!("Shutdown signal recevied...");
            drop(status_ch);
            drop(notify_shutdown);
            drop(shutdown_complete_tx);
            let _ = shutdown_complete_rx.recv().await;
            return;
        }
        let mut client  = Client{
            remote_address: address.clone() ,
            sh: Shutdown::new(notify_shutdown.subscribe())
        };
        let status_ch_clone  = status_ch.clone();
        
        tokio::spawn(async move {
            if let Err(e) = client.connect(id,status_ch_clone).await {
                debug!("Not able to connect to server with error {:?}",e)
            };
        });
        debug!("Established connection:{:?}",runner);
        thread::sleep(Duration::from_millis(delay));
    }
    debug!("Shutting down the all the clients...");
    drop(status_ch);
    drop(notify_shutdown);
    // Drop final `Sender` so the `Receiver` below can complete
    drop(shutdown_complete_tx);

    // Wait for all active connections to finish processing. As the `Sender`
    // handle held by the listener has been dropped above, the only remaining
    // `Sender` instances are held by connection handler tasks. When those drop,
    // the `mpsc` channel will close and `recv()` will return `None`.
    let _ = shutdown_complete_rx.recv().await;
    // Stop the report
    stop.store(true,Relaxed);
    debug!("Shutdown completed");
}

impl Client{
    pub async fn connect(&mut self,id:u64,status_ch:mpsc::Sender<TcpClientConnectionMetrics>) -> Result<(),Box<dyn std::error::Error>>{
            //setting up the connection time out
            const CONNECTION_TIME:u64 = 100;

            //Initialize the connection metrics
            let mut connection_metrics = TcpClientConnectionMetrics::new(id,ConnectionStatus::ConnectionDialing);
            report_tcp_client_connection_metrics(connection_metrics,status_ch.clone()).await;

            let start = Instant::now();
            let socket = match timeout (
                Duration::from_secs(CONNECTION_TIME),
                TcpStream::connect(&self.remote_address)
            ).await {
                Ok(v) => match v  {
                    Ok(s) => {
                        let time_taken = start.elapsed().as_micros();
                        connection_metrics.metrics.tcp_established_duration = time_taken as u64;
                        connection_metrics.status =  ConnectionStatus::ConnectionEstablished;
                        report_tcp_client_connection_metrics(connection_metrics,status_ch.clone()).await;
                        s                    
                    },
                    Err(e) => {
                        let time_taken = start.elapsed().as_micros();
                        connection_metrics.metrics.tcp_errored_duration = time_taken as u64;
                        connection_metrics.status  = ConnectionStatus::ConnectionError;
                        report_tcp_client_connection_metrics(connection_metrics,status_ch).await;
                        return Err(e.into())
                    }
                },
                Err(e) => return Err(format!("timeout while connecting to server:{}",e).into())
            };


            let mut connection = Connection::new(socket);
            while !self.sh.is_shutdown(){
                let _ = tokio::select!{
                    res = connection.read_stream() => match res{
                        Ok(_) => {},
                        Err(e) => {
                            connection_metrics.status =  ConnectionStatus::ConnectionClosed;
                            println!("Connection read error :{:?}",e);
                            report_tcp_client_connection_metrics(connection_metrics,status_ch.clone()).await;
                        }
                    },
                    _ = self.sh.recv() => {
                        connection_metrics.status = ConnectionStatus::ConnectionClosed;
                        report_tcp_client_connection_metrics(connection_metrics,status_ch).await;
                        println!("client shutdown");
                        info!("Shutting down the  client");
                        return Ok(())
                    }
                };
            }
            Ok(())
    }
}
