use crate::{
    net::{
        tcp::TcpStream,
        tls::{
            server::TlsStream,
            TlsAcceptor,
        },
    },
    prelude::*,
};
use async_std::{
    io,
    task,
};
use std::{
    sync::Arc,
    task::{
        Context,
        Poll,
    },
};

const SLICE_CAPACITY: usize = 1024;

#[derive(Default)]
struct ConnectionHandlerState {
    tls: Option<TlsAcceptor>,
}

impl ConnectionHandlerState {
    #[inline]
    pub fn tls(&self) -> Option<&TlsAcceptor> {
        self.tls.as_ref()
    }
}

#[derive(Default)]
pub struct ConnectionHandlerBuilder(ConnectionHandlerState);

impl ConnectionHandlerBuilder {
    pub fn with_tls<Value>(mut self, value: Value) -> Self
    where
        TlsAcceptor: From<Value>,
    {
        self.0.tls = Some(TlsAcceptor::from(value));
        self
    }

    pub fn build(self) -> ConnectionHandler {
        ConnectionHandler::from(self.0)
    }
}

#[derive(Default)]
pub struct ConnectionHandler(Arc<ConnectionHandlerState>);

impl ConnectionHandler {
    pub fn new() -> ConnectionHandlerBuilder {
        ConnectionHandlerBuilder::default()
    }

    #[instrument(skip(self))]
    pub fn handle_connection(&self, stream: TcpStream) {
        let this = self.clone();
        task::spawn(async move {
            let mut connection = this.accept(stream).await?;
            let mut slices = [[0u8; SLICE_CAPACITY]; 4];
            let mut bufs = slices
                .iter_mut()
                .map(|slice| io::IoSliceMut::new(slice))
                .collect::<Vec<_>>();

            connection.read_vectored(&mut bufs).await?;

            let mut buf = String::new();
            for slice in slices.iter() {
                let s = String::from_utf8(slice.to_vec())
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
                buf.push_str(&s);
            }
            info!("{}", buf);
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            connection.write_all(response.as_bytes()).await?;
            connection.flush().await?;

            io::Result::Ok(())
        });
    }

    #[instrument(err, skip(self))]
    async fn accept(&self, stream: TcpStream) -> io::Result<Connection> {
        if let Some(acceptor) = self.0.tls() {
            let inner = acceptor.accept(stream).await?;
            Ok(Connection::from(inner))
        } else {
            Ok(Connection::from(stream))
        }
    }
}

impl Clone for ConnectionHandler {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl From<ConnectionHandlerState> for ConnectionHandler {
    fn from(state: ConnectionHandlerState) -> Self {
        Self(state.into())
    }
}

#[pin_project::pin_project(project = PinProjectedConnection)]
pub enum Connection {
    Secured(#[pin] TlsStream<TcpStream>),
    Raw(#[pin] TcpStream),
}

impl From<TlsStream<TcpStream>> for Connection {
    fn from(inner: TlsStream<TcpStream>) -> Self {
        Self::Secured(TlsStream::from(inner))
    }
}

impl From<TcpStream> for Connection {
    fn from(stream: TcpStream) -> Self {
        Self::Raw(stream)
    }
}

impl AsyncRead for Connection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match Self::project(self) {
            PinProjectedConnection::Secured(inner) => inner.poll_read(cx, buf),
            PinProjectedConnection::Raw(inner) => inner.poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match Self::project(self) {
            PinProjectedConnection::Secured(inner) => inner.poll_write(cx, buf),
            PinProjectedConnection::Raw(inner) => inner.poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Self::project(self) {
            PinProjectedConnection::Secured(inner) => inner.poll_flush(cx),
            PinProjectedConnection::Raw(inner) => inner.poll_flush(cx),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Self::project(self) {
            PinProjectedConnection::Secured(inner) => inner.poll_close(cx),
            PinProjectedConnection::Raw(inner) => inner.poll_close(cx),
        }
    }
}
