use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};

use super::{errors::Result, request::Request, response::Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub credentials: Option<Credentials>,
    pub heartbeat: Heartbeat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 61616,
            credentials: None,
            heartbeat: Heartbeat::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Heartbeat {
    pub rx: u32,
    pub tx: u32,
}

pub struct Connection {
    stream: TcpStream,
    local: SocketAddr,
    peer: SocketAddr,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.local, self.peer)
    }
}

impl Connection {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let local = stream.local_addr()?;
        let peer = stream.peer_addr()?;
        let it = Self {
            stream: stream,
            local: local,
            peer: peer,
        };
        debug!("open {}", it);
        Ok(it)
    }
    pub fn send<R: Request>(&mut self, req: &R) -> Result<()> {
        let buf = req.to_string();
        debug!("send {} to {}", buf, self.peer);
        self.stream.write_all(&buf.as_bytes())?;
        Ok(())
    }
    pub fn receive<R: Response>(&mut self) -> Result<R> {
        let mut buf = String::new();
        self.stream.read_to_string(&mut buf)?;
        debug!("receive {} from {}", buf, self.peer);
        Ok(buf.parse()?)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        debug!("shutdown {}", self);
        if let Err(e) = self.stream.shutdown(Shutdown::Both) {
            error!("{}", e.description())
        }
    }
}
