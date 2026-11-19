use core::{fmt, future::Future, pin::Pin, time::Duration};
use std::error::Error;

use bytes::Bytes;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) use tokio::time::Instant;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) use web_time::Instant;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransportKind {
    Hyper,
    Custom,
}

#[must_use]
pub const fn transport_kind() -> TransportKind {
    #[cfg(feature = "hyper")]
    {
        TransportKind::Hyper
    }
    #[cfg(not(feature = "hyper"))]
    {
        TransportKind::Custom
    }
}

pub trait Transport: Send + Sync + 'static {
    fn send(
        &self,
        request: http::Request<Body>,
    ) -> BoxFuture<'_, Result<http::Response<Bytes>, Box<dyn Error + Send + Sync>>>;

    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()>;
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Body {
    chunks: Vec<Bytes>,
}

impl Body {
    pub(crate) const fn new(chunks: Vec<Bytes>) -> Self {
        Self { chunks }
    }

    #[must_use]
    pub fn chunks(&self) -> &[Bytes] {
        &self.chunks
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.chunks.iter().map(Bytes::len).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chunks.iter().all(Bytes::is_empty)
    }

    #[must_use]
    pub fn into_bytes(mut self) -> Bytes {
        match self.chunks.len() {
            0 => Bytes::new(),
            1 => self.chunks.swap_remove(0),
            _ => self.chunks.concat().into(),
        }
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body")
            .field("chunks", &self.chunks.len())
            .field("len", &self.len())
            .finish()
    }
}

pub(crate) async fn sleep_until(transport: &dyn Transport, deadline: Instant) {
    let now = Instant::now();
    if deadline > now {
        transport.sleep(deadline - now).await;
    }
}

#[cfg(feature = "hyper")]
pub(crate) mod hyper {
    use core::{
        convert::Infallible,
        pin::Pin,
        task::{Context, Poll},
        time::Duration,
    };
    use std::error::Error;

    use bytes::Bytes;
    use http_body_util::BodyExt;
    use hyper::body::{Frame, SizeHint};
    use hyper_rustls::HttpsConnector;
    use hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::{TokioExecutor, TokioTimer},
    };

    use super::{Body, BoxFuture, Transport};

    pub(crate) struct HyperTransport {
        client: Client<HttpsConnector<HttpConnector>, HyperBody>,
    }

    impl HyperTransport {
        pub(crate) fn new() -> Self {
            let connector = hyper_rustls::HttpsConnectorBuilder::new()
                .with_tls_config(crate::tls::client_config())
                .https_or_http()
                .enable_all_versions()
                .build();
            let client = Client::builder(TokioExecutor::new())
                .timer(TokioTimer::new())
                .pool_timer(TokioTimer::new())
                .build(connector);
            Self { client }
        }
    }

    impl Transport for HyperTransport {
        fn send(
            &self,
            request: http::Request<Body>,
        ) -> BoxFuture<'_, Result<http::Response<Bytes>, Box<dyn Error + Send + Sync>>> {
            let response = self.client.request(request.map(HyperBody::from));
            Box::pin(async move {
                let (parts, body) = response.await?.into_parts();
                let body = body.collect().await?.to_bytes();
                Ok(http::Response::from_parts(parts, body))
            })
        }

        fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
            Box::pin(tokio::time::sleep(duration))
        }
    }

    #[derive(Debug)]
    pub(crate) struct HyperBody {
        chunks: std::vec::IntoIter<Bytes>,
        remaining: u64,
    }

    impl From<Body> for HyperBody {
        fn from(body: Body) -> Self {
            Self {
                remaining: body.len() as u64,
                chunks: body.chunks.into_iter(),
            }
        }
    }

    impl hyper::body::Body for HyperBody {
        type Data = Bytes;
        type Error = Infallible;

        fn poll_frame(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
            let this = self.get_mut();
            let next = this.chunks.by_ref().find(|chunk| !chunk.is_empty());
            if let Some(chunk) = &next {
                this.remaining -= chunk.len() as u64;
            }
            Poll::Ready(next.map(|chunk| Ok(Frame::data(chunk))))
        }

        fn is_end_stream(&self) -> bool {
            self.remaining == 0
        }

        fn size_hint(&self) -> SizeHint {
            SizeHint::with_exact(self.remaining)
        }
    }

    #[cfg(test)]
    mod tests {
        use hyper::body::Body as _;

        use super::*;

        #[tokio::test]
        async fn sends_every_non_empty_chunk_with_the_exact_length() {
            let body = Body::new(vec![
                Bytes::from_static(b"ab"),
                Bytes::new(),
                Bytes::from_static(b"c"),
            ]);
            let mut body = HyperBody::from(body);
            assert_eq!(body.size_hint().exact(), Some(3));
            let mut frames = Vec::new();
            while let Some(frame) = body.frame().await {
                frames.push(frame.unwrap().into_data().unwrap());
            }
            assert_eq!(frames, [&b"ab"[..], b"c"]);
            assert!(body.is_end_stream());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_body_joins_its_chunks_only_when_needed() {
        let one = Bytes::from_static(b"{}");
        let body = Body::new(vec![one.clone()]);
        assert_eq!(body.len(), 2);
        assert!(!body.is_empty());

        assert_eq!(body.into_bytes().as_ptr(), one.as_ptr());

        let body = Body::new(vec![
            Bytes::from_static(b"ab"),
            Bytes::new(),
            Bytes::from_static(b"c"),
        ]);
        assert_eq!(body.chunks().len(), 3);
        assert_eq!(body.len(), 3);
        assert_eq!(body.into_bytes(), "abc");
        assert!(Body::default().is_empty());
        assert!(Body::new(vec![Bytes::new()]).is_empty());
        assert_eq!(Body::default().into_bytes(), "");
    }
}
