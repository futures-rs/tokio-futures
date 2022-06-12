use std::{io::Result, pin::Pin, task::Poll};

use tokio::io::{AsyncRead, AsyncWrite};

pub struct TokioAsyncReadWrite<T>
where
    T: AsyncRead + AsyncWrite,
{
    inner: Pin<Box<T>>,
}

impl<T> TokioAsyncReadWrite<T>
where
    T: AsyncRead + AsyncWrite,
{
    pub fn new(inner: T) -> Self {
        TokioAsyncReadWrite::<T> {
            inner: Box::pin(inner),
        }
    }
}

impl<T> From<T> for TokioAsyncReadWrite<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn from(inner: T) -> Self {
        TokioAsyncReadWrite::<T>::new(inner)
    }
}

impl<T> futures::AsyncRead for TokioAsyncReadWrite<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let mut read_buf = tokio::io::ReadBuf::new(buf);
        match self.inner.as_mut().poll_read(cx, &mut read_buf) {
            Poll::Pending => {
                return Poll::Pending;
            }
            Poll::Ready(_) => {
                return Poll::Ready(Ok(read_buf.filled().len()));
            }
        }
    }
}

impl<T> futures::AsyncWrite for TokioAsyncReadWrite<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<()>> {
        self.inner.as_mut().poll_shutdown(cx)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<()>> {
        self.inner.as_mut().poll_flush(cx)
    }
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        self.inner.as_mut().poll_write(cx, buf)
    }
}
