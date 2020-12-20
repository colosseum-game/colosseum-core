use bytes::{
    Buf,
    Bytes,
    BytesMut,
};

use crate::message::Message;

use std::{
    convert::{
        Infallible,
        TryInto,
    },
    fmt,
    io::Cursor,
    net::SocketAddr,
    num::TryFromIntError,
};

use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::TcpStream,
};

#[derive(Debug)]
pub enum Error {
    Closed,
    IOError(std::io::Error),
    ParseError(String),
}

impl From<Infallible> for Error {
    fn from(_src: Infallible) -> Error {
        Error::Closed
    }
}

impl From<bincode::Error> for Error {
    fn from(src: bincode::Error) -> Error {
        Error::ParseError(format!("{}", src))
    }
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::ParseError(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        src.to_string().into()
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Closed => "connection was closed".fmt(fmt),
            IOError(e) => e.fmt(fmt),
            ParseError(e) => e.fmt(fmt),
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub addr: SocketAddr,
    pub stream: TcpStream,
    pub buffer: BytesMut,
}

impl Connection {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Connection {
        Connection {
            addr,
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_message(&mut self) -> Result<Option<Message>, Error> {
        // attempt to read data into the buffer
        // when the following condition is true, the connection has been closed
        if self.stream.read_buf(&mut self.buffer).await? == 0 { return Err(Error::Closed); }

        let mut buf = Cursor::new(&self.buffer[..]);

        if buf.remaining() < std::mem::size_of::<u16>() { return Ok(None); }
        let len = buf.get_u16() as usize;

        if buf.remaining() < len { return Ok(None); }
        let data = Bytes::copy_from_slice(&buf.bytes()[..len]);
        let message = bincode::deserialize(&data)?;

        self.buffer.advance(len);

        Ok(Some(message))
    }

    pub async fn write_message(&mut self, message: &Message) -> Result<(), Error> {
        let data = bincode::serialize(message)?;
        let len = data.len();

        self.stream.write_u16(len.try_into().unwrap()).await?;
        self.stream.write_all(&data).await?;
        self.stream.flush().await?;

        Ok(())
    }
}
