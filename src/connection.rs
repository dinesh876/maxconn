use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use socket2::TcpKeepalive;
use std::time::Duration;
use std::net::SocketAddr;
#[derive(Debug)]
pub struct Connection{
    stream: TcpStream,
    buffer: [u8;1],
}


impl Connection {

    pub fn new(socket: TcpStream) -> Self{
        Connection {
            stream: socket,
            buffer: [0u8;1],
        }
    }

    pub fn peer_address(&mut self) -> Result<SocketAddr,Box<dyn std::error::Error>>{
        match self.stream.peer_addr() {
            Ok(addr) => Ok(addr),
            Err(e) => Err(e.into())
        }
    }
    pub fn set_keepalive(&mut self)-> Result<(),Box<dyn std::error::Error>> {
        let socket: socket2::SockRef = socket2::SockRef::from(&self.stream);
        let keepalive = TcpKeepalive::new()
            .with_time(Duration::from_secs(4))
            .with_interval(Duration::from_secs(1))
            .with_retries(4);
        socket.set_tcp_keepalive(&keepalive)?;
        Ok(())
    }

    pub async  fn read_stream(&mut self) -> Result<(),std::io::Error> {
        let mut err: Option<std::io::Error> = None;
        loop{
            let n = match self.stream.read_exact(&mut self.buffer).await {
                Ok(v) => v,
                Err(e) => {
                    err = Some(e);
                    break;
                }
            };
            if n == 0 {
                break;
            }
        }
        if err.is_some() {
            Err(err.unwrap())
        } else {
            Ok(())
        }
    } 

}
