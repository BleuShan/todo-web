use crate::{
    net::{
        tcp::TcpStream,
        tls::{
            server,
            TlsAcceptor,
            TlsStream,
        },
    },
    prelude::*,
};
use bytes::BytesMut;
use std::{
    sync::Arc,
    task::{
        Context,
        Poll,
    },
};

const BUFFER_CAPACITY: usize = 8 * 1024;

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
        tokio::spawn(async move {
            let mut connection = this.accept(stream).await?;
            let mut buf = BytesMut::with_capacity(BUFFER_CAPACITY);

            connection.read_buf(&mut buf).await?;
            info!(
                "{}",
                String::from_utf8(buf.freeze().to_vec())
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?
            );
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            connection.write_all(response.as_bytes()).await?;
            connection.flush().await?;

            io::Result::Ok(())
        });
    }

    #[instrument(err, skip(self))]
    async fn accept(&self, stream: TcpStream) -> io::Result<Connection<TcpStream>> {
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
pub enum Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    Secured(#[pin] TlsStream<Inner>),
    Raw(#[pin] Inner),
}

impl<Inner> From<server::TlsStream<Inner>> for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    fn from(inner: server::TlsStream<Inner>) -> Self {
        Self::Secured(TlsStream::from(inner))
    }
}

impl<Inner> From<TlsStream<Inner>> for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    fn from(inner: TlsStream<Inner>) -> Self {
        Self::Secured(inner)
    }
}

impl<Inner> From<Inner> for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    fn from(stream: Inner) -> Self {
        Self::Raw(stream)
    }
}

impl<Inner> Deref for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        match self {
            Connection::Secured(inner) => {
                let (inner, _) = inner.get_ref();
                inner
            }
            Connection::Raw(inner) => inner,
        }
    }
}

impl<Inner> DerefMut for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Connection::Secured(inner) => {
                let (inner, _) = inner.get_mut();
                inner
            }
            Connection::Raw(inner) => inner,
        }
    }
}

impl<Inner> AsyncRead for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
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

impl<Inner> AsyncWrite for Connection<Inner>
where
    Inner: AsyncRead + AsyncWrite + Unpin,
{
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

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Self::project(self) {
            PinProjectedConnection::Secured(inner) => inner.poll_shutdown(cx),
            PinProjectedConnection::Raw(inner) => inner.poll_shutdown(cx),
        }
    }
}
