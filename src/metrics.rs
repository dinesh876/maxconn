#[derive(Debug)]
pub enum ConnectionStatus {
    ConnectionNotInitiated,
    ConnectionDialing,
    ConnectionEstablished,
    ConnectionClosed,
    ConnectionError
}


#[derive(Debug)]
pub struct ConnectionMetrics{
    pub tcp_established_duration: u64,
    pub tcp_errored_duration: u64,
}

#[derive(Debug)]
pub struct Metrics{
    pub id: u64,
    pub status: ConnectionStatus,
    pub metrics: ConnectionMetrics
}

impl Metrics {
    pub fn new(id: u64,status: ConnectionStatus) -> Self {
        Metrics {
            id,
            status,
            metrics: ConnectionMetrics{
                tcp_established_duration: 0,
                tcp_errored_duration:0
            }
        }
    }
}
