use std::fmt;
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum ConnectionStatus {
    ConnectionNotInitiated,
    ConnectionDialing,
    ConnectionEstablished,
    ConnectionClosed,
    ConnectionError
}


#[derive(Debug,Copy,Clone)]
pub struct ConnectionMetrics{
    pub tcp_established_duration: u64,
    pub tcp_errored_duration: u64,
}

#[derive(Debug,Copy,Clone)]
pub struct TcpClientConnectionMetrics{
    pub id: u64,
    pub status: ConnectionStatus,
    pub metrics: ConnectionMetrics
}

impl fmt::Display for TcpClientConnectionMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID- {} : status - {:?}:  metrics- {:?}", self.id, self.status,self.metrics)
    }
}

impl TcpClientConnectionMetrics {
    pub fn new(id: u64,status: ConnectionStatus) -> Self {
        TcpClientConnectionMetrics {
            id,
            status,
            metrics: ConnectionMetrics{
                tcp_established_duration: 0,
                tcp_errored_duration:0
            }
        }
    }
    pub fn get_connection_status(&self) -> ConnectionStatus {
        self.status
    }
    pub fn get_tcp_processing_duration(&self) -> u64{
        if went_ok(&self){
            return self.metrics.tcp_established_duration;
        }
        return self.metrics.tcp_errored_duration;
    }
    fn is_status_in(&self,statuses:Vec<ConnectionStatus>)->bool{
        for status in statuses {
            if self.get_connection_status() == status {
                return true
            }
        }
        return false
    }
}

pub fn went_ok(c:&TcpClientConnectionMetrics) -> bool {
   c.is_status_in(vec![ConnectionStatus::ConnectionEstablished,ConnectionStatus::ConnectionClosed]) 
}

pub fn is_ok(c:&TcpClientConnectionMetrics) -> bool {
   c.is_status_in(vec![ConnectionStatus::ConnectionEstablished]) 
}


pub fn with_error(c:&TcpClientConnectionMetrics) -> bool {
   c.is_status_in(vec![ConnectionStatus::ConnectionError]) 
}

pub fn pending_to_process(c:&TcpClientConnectionMetrics) -> bool {
   c.is_status_in(vec![ConnectionStatus::ConnectionNotInitiated,ConnectionStatus::ConnectionDialing]) 
}
