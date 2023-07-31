use tokio::net::{TcpStream,TcpListener};
use std::sync::Arc;
use tokio::sync::{mpsc,broadcast,Semaphore};
use tokio::time::{Duration,self};
use crate::{Shutdown,Connection};
use std::future::Future;
use tracing::{debug,info};

#[derive(Debug)]
struct Listener {
    listener: TcpListener,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

#[derive(Debug)]
struct Handler{
    connection: Connection,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
}


pub async fn run(listener: TcpListener,max_connection:usize,shutdown:impl Future){
    let (notify_shutdown,_) = broadcast::channel(1);
    let (shutdown_complete_tx,mut shutdown_complete_rx) = mpsc::channel(1);

    //Initialize the listener

    let mut server = Listener{
        listener,
        limit_connections: Arc::new(Semaphore::new(max_connection)),
        notify_shutdown,
        shutdown_complete_tx,
    };
    
    tokio::select! {
        res = server.run() => {
            if let Err(e) = res {
                println!("{:?}",e)
            } 
        }
        _ = shutdown => {
            info!("Shut down signal Recevied...")
        }
    }

    let Listener{notify_shutdown,shutdown_complete_tx,..} = server;
    drop(notify_shutdown);
    drop(shutdown_complete_tx);
    let _ = shutdown_complete_rx.recv().await;

}



impl Listener {
    pub async fn run(&mut self) -> Result<(),Box<dyn std::error::Error>> {
        info!("Listening on {:?}", self.listener.local_addr().unwrap());
        info!("Accepting inbound connection...");
        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();
            let socket = self.accept().await?;
            
            let mut handler = Handler{
                connection: Connection::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                _shutdown_complete: self.shutdown_complete_tx.clone()
            };
            tokio::spawn(async move{
                if let Err(e) = handler.run().await {
                    debug!("peer: {:?} - connection closed with error {:?}:",handler.connection.peer_address().unwrap(),e);
                };
                drop(permit);
            });

        }
        
    }

    async fn accept(&mut self) -> Result<TcpStream,Box<dyn std::error::Error>>{
        let mut backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket,_)) => return Ok(socket),
                Err(err) =>{
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;
            backoff *= 2;
        }
    }
}

impl Handler {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>{
            let peer_address =  self.connection.peer_address()?;
            debug!(%peer_address,"Recevied a  connection");
            self.connection.set_keepalive()?;
            while !self.shutdown.is_shutdown(){
                let _ = tokio::select!{
                    res = self.connection.read_stream() => res?,
                    _ = self.shutdown.recv() => {
                        info!("shutting down the client: {}",peer_address);
                        return Ok(());
                    }
                };
            }
            Ok(())
    }
}


