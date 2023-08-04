use crate::TcpClientConnectionMetrics;
#[derive(Debug)]
pub struct GroupOfConnections {
    connections: Vec<TcpClientConnectionMetrics>,
    max_concurrent_established: u64
}

impl GroupOfConnections{
    pub fn new(num_of_connection:u64) -> Self {
        GroupOfConnections{
            connections: Vec::with_capacity(num_of_connection),
            max_concurrent_established: 0
        }
    }
}
