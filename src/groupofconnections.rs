use crate::{TcpClientConnectionMetrics,ConnectionStatus};
use std::fmt;
#[derive(Debug)]
pub struct GroupOfConnections {
    pub connections: Vec<TcpClientConnectionMetrics>,
    pub max_concurrent_established: u64,
    pub connection_dialing: u64,
    pub connection_witherror: u64,
    pub connection_closed:u64
}

impl GroupOfConnections{
    pub fn new(num_of_connection:u64) -> Self {
        GroupOfConnections{
            connections: Vec::with_capacity(num_of_connection.try_into().unwrap()),
            max_concurrent_established: 0,
            connection_dialing: 0,
            connection_witherror:0,
            connection_closed:0

        }
    }
}

impl fmt::Display for GroupOfConnections {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let (mut n_dialing, mut n_established, mut n_closed,mut n_not_initiated,mut n_error,mut n_total) = (0,0,0,0,0,0);
        for conn in &self.connections {
           match conn.status {
                ConnectionStatus::ConnectionNotInitiated => n_not_initiated += 1,
                ConnectionStatus::ConnectionDialing => n_dialing += 1,
                ConnectionStatus::ConnectionEstablished => n_established += 1,
                ConnectionStatus::ConnectionError => n_error += 1,
                ConnectionStatus::ConnectionClosed => n_closed += 1,
           } 
           n_total += 1;
        }
        write!(f,"Total: {} , Dialing: {}, Established: {}, Closed: {}, Error: {}, NotInitiated: {}",n_total,n_dialing,n_established,n_closed,n_error,n_not_initiated)
    }
}
