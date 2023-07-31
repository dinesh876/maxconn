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
use crate::{Metrics,ConnectionStatus};

pub struct Client{
    remote_address:String,
    sh: Shutdown,
}

pub async fn run(address: String,shutdown: impl Future + Send + 'static,max_connections: u64,delay: u64){

    let stop:Arc<AtomicBool> =  Arc::new(AtomicBool::new(false));
    let stop_clone =  stop.clone();
    let (notify_shutdown,_) = broadcast::channel(1);
    let notify_shutdown_clone = notify_shutdown.clone();
    tokio::spawn(async move {
        tokio::select!{
            _ = shutdown => {
                println!("shutting down...")
            } 
        }
        stop_clone.store(true,Relaxed);
        drop(notify_shutdown_clone);
    });

    for runner in 0..max_connections {
        let id = runner;

        if stop.clone().load(Relaxed) {return ;}
        let mut client  = Client{
            remote_address: address.clone() ,
            sh: Shutdown::new(notify_shutdown.subscribe())
        };

        tokio::spawn(async move {
            if let Err(e) = client.connect(id).await {
                println!("Error {:?}",e)
            };
        });
        println!("Established connection:{:?}",runner);
        thread::sleep(Duration::from_secs(delay));
    }
    println!("Shutting down the all the clients");
    drop(notify_shutdown)

}

impl Client{
    pub async fn connect(&mut self,id:u64) -> Result<(),Box<dyn std::error::Error>>{
            const CONNECTION_TIME:u64 = 100;
            let mut connection_details = Metrics::new(id,ConnectionStatus::ConnectionDialing);
            let start = Instant::now();
            let socket = match timeout (
                Duration::from_secs(CONNECTION_TIME),
                TcpStream::connect(&self.remote_address)
            ).await {
                Ok(v) => match v  {
                    Ok(s) => {
                        let time_taken = start.elapsed().as_micros();
                        connection_details.metrics.tcp_established_duration = time_taken as u64;
                        connection_details.status =  ConnectionStatus::ConnectionEstablished;
                        s                    
                    },
                    Err(e) => {
                        let time_taken = start.elapsed().as_micros();
                        connection_details.metrics.tcp_errored_duration = time_taken as u64;
                        connection_details.status  = ConnectionStatus::ConnectionError;
                        panic!("{}",format!("Error while connecting to server: {}",e))
                    }
                },
                Err(e) => panic!("{}",format!("timeout while connecting to server:{}",e))
            };


            let mut connection = Connection::new(socket);
            while !self.sh.is_shutdown(){
                let _ = tokio::select!{
                    res = connection.read_stream() => match res{
                        Ok(_) => {},
                        Err(_) => {
                            connection_details.status =  ConnectionStatus::ConnectionClosed;
                        }
                    },
                    _ = self.sh.recv() => {
                        println!("Shutting down the  client");
                        return Ok(())
                    }
                };
            }

            
            println!("{:?}",connection_details);
            Ok(())
    }
}
