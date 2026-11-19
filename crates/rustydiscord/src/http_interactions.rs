use core::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use std::{
    sync::{Arc, Mutex, PoisonError},
    task::Wake,
};

use ed25519_dalek::{Signature, VerifyingKey};
use rustydiscord_cache::{Backend, DashMapBackend};
use rustydiscord_model::{
    gateway::payload::InteractionCreate,
    id::InteractionId,
    interaction::{InteractionData, InteractionResponse},
    util::Timestamp,
};
use rustydiscord_rest::{Error, Rest};
use tokio::sync::oneshot;

use crate::Client;

const MAX_AGE_SECS: u64 = 5 * 60;

const PONG: &[u8] = br#"{"type":1}"#;

type Background = Pin<Box<dyn Future<Output = ()> + Send>>;

pub(crate) enum Initial {
    Body(Vec<u8>),

    Rest,
}

pub(crate) type InitialSender = oneshot::Sender<Initial>;

pub(crate) type InitialReceiver = oneshot::Receiver<Initial>;

pub(crate) struct Inline {
    interaction: InteractionId,
    slot: Mutex<Option<InitialSender>>,
}

impl Inline {
    pub(crate) fn new(interaction: InteractionId) -> (Self, InitialReceiver) {
        let (sender, receiver) = oneshot::channel();
        let inline = Self {
            interaction,
            slot: Mutex::new(Some(sender)),
        };
        (inline, receiver)
    }

    pub(crate) fn take(&self, interaction: InteractionId) -> Option<InitialSender> {
        if interaction != self.interaction {
            return None;
        }
        self.slot
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }
}

pub(crate) async fn respond(
    rest: &Rest,
    initial: InitialSender,
    interaction: InteractionId,
    token: &str,
    response: &InteractionResponse,
) -> Result<(), Error> {
    if response.files().is_empty()
        && let Ok(body) = serde_json::to_vec(response)
    {
        let _ = initial.send(Initial::Body(body));
        return Ok(());
    }
    rest.create_interaction_response(interaction, token, response)
        .await?;
    let _ = initial.send(Initial::Rest);
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct HttpResponse {
    pub status: u16,

    pub body: Vec<u8>,
}

impl HttpResponse {
    const fn empty(status: u16) -> Self {
        Self {
            status,
            body: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("the public key is not 64 hexadecimal digits of a valid Ed25519 key")]
pub struct InvalidPublicKey;

pub struct InteractionsEndpoint<B: Backend = DashMapBackend> {
    client: Client<B>,
    key: VerifyingKey,
}

impl<B: Backend> Clone for InteractionsEndpoint<B> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            key: self.key,
        }
    }
}

impl<B: Backend> fmt::Debug for InteractionsEndpoint<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InteractionsEndpoint")
            .field("client", &self.client)
            .finish_non_exhaustive()
    }
}

impl<B: Backend> InteractionsEndpoint<B> {
    pub(crate) fn new(client: Client<B>, public_key: &str) -> Result<Self, InvalidPublicKey> {
        let bytes = decode_hex::<32>(public_key).ok_or(InvalidPublicKey)?;
        let key = VerifyingKey::from_bytes(&bytes).map_err(|_| InvalidPublicKey)?;
        Ok(Self { client, key })
    }

    pub async fn handle(
        &self,
        signature: &str,
        timestamp: &str,
        body: &[u8],
    ) -> (
        HttpResponse,
        impl Future<Output = ()> + Send + 'static + use<B>,
    ) {
        let idle = || -> Background { Box::pin(core::future::ready(())) };
        if !verify(
            &self.key,
            signature,
            timestamp,
            body,
            Timestamp::now().as_unix_secs(),
        ) {
            return (HttpResponse::empty(401), idle());
        }
        let event: InteractionCreate = match serde_json::from_slice(body) {
            Ok(event) => event,
            Err(error) => {
                tracing::warn!(%error, "a signed HTTP interaction could not be decoded");
                return (HttpResponse::empty(400), idle());
            }
        };
        if matches!(event.data, InteractionData::Ping) {
            let pong = HttpResponse {
                status: 200,
                body: PONG.to_vec(),
            };
            return (pong, idle());
        }
        let (mut initial, work) = self.client.dispatch_http(event);
        let mut work = Relayed::new(work);
        tokio::select! {
            biased;
            result = &mut initial => (answer(result.ok()), Box::pin(work)),
            () = &mut work => (answer(initial.try_recv().ok()), idle()),
        }
    }
}

struct Relayed {
    future: Background,
    relay: Arc<Relay>,
    waker: Waker,
}

struct Relay(Mutex<Option<Waker>>);

impl Wake for Relay {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(waker) = &*self.0.lock().unwrap_or_else(PoisonError::into_inner) {
            waker.wake_by_ref();
        }
    }
}

impl Relayed {
    fn new(future: Background) -> Self {
        let relay = Arc::new(Relay(Mutex::new(None)));
        let waker = Waker::from(Arc::clone(&relay));
        Self {
            future,
            relay,
            waker,
        }
    }
}

impl Future for Relayed {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        {
            let mut last = this.relay.0.lock().unwrap_or_else(PoisonError::into_inner);
            if !last
                .as_ref()
                .is_some_and(|waker| waker.will_wake(cx.waker()))
            {
                *last = Some(cx.waker().clone());
            }
        }
        this.future
            .as_mut()
            .poll(&mut Context::from_waker(&this.waker))
    }
}

fn answer(initial: Option<Initial>) -> HttpResponse {
    match initial {
        Some(Initial::Body(body)) => HttpResponse { status: 200, body },
        Some(Initial::Rest) => HttpResponse::empty(202),
        None => {
            tracing::warn!("the handlers of an HTTP interaction finished without responding");
            HttpResponse::empty(500)
        }
    }
}

fn verify(key: &VerifyingKey, signature: &str, timestamp: &str, body: &[u8], now: i64) -> bool {
    let Ok(signed_at) = timestamp.parse::<i64>() else {
        return false;
    };
    if now.abs_diff(signed_at) > MAX_AGE_SECS {
        return false;
    }
    let Some(signature) = decode_hex::<64>(signature) else {
        return false;
    };
    let mut message = Vec::with_capacity(timestamp.len() + body.len());
    message.extend_from_slice(timestamp.as_bytes());
    message.extend_from_slice(body);
    key.verify_strict(&message, &Signature::from_bytes(&signature))
        .is_ok()
}

fn decode_hex<const N: usize>(hex: &str) -> Option<[u8; N]> {
    const fn digit(c: u8) -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    }
    let hex = hex.as_bytes();
    if hex.len() != 2 * N {
        return None;
    }
    let mut out = [0; N];
    for (byte, pair) in out.iter_mut().zip(hex.as_chunks::<2>().0) {
        *byte = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;

    use ed25519_dalek::{Signer, SigningKey};

    use super::*;

    const NOW: i64 = 1_800_000_000;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().fold(String::new(), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
    }

    fn key() -> SigningKey {
        SigningKey::from_bytes(&[7; 32])
    }

    fn sign(timestamp: &str, body: &[u8]) -> String {
        let mut message = timestamp.as_bytes().to_vec();
        message.extend_from_slice(body);
        hex(&key().sign(&message).to_bytes())
    }

    #[test]
    fn decodes_hex_of_the_exact_length_only() {
        assert_eq!(decode_hex::<2>("0aFf"), Some([0x0a, 0xff]));
        assert_eq!(decode_hex::<2>("0aF"), None);
        assert_eq!(decode_hex::<2>("0aFf0"), None);
        assert_eq!(decode_hex::<2>("0aFg"), None);
        assert_eq!(decode_hex::<1>("é"), None);
    }

    #[test]
    fn verifies_the_signature_of_the_timestamp_and_body() {
        let public = key().verifying_key();
        let (timestamp, body) = ("1800000000", br#"{"type":1}"#.as_slice());
        let signature = sign(timestamp, body);
        assert!(verify(&public, &signature, timestamp, body, NOW));
        assert!(verify(
            &public,
            &signature.to_uppercase(),
            timestamp,
            body,
            NOW
        ));

        assert!(!verify(
            &public,
            &signature,
            timestamp,
            br#"{"type":2}"#,
            NOW
        ));
        assert!(!verify(&public, &signature, "1800000001", body, NOW));
        let other = SigningKey::from_bytes(&[8; 32]).verifying_key();
        assert!(!verify(&other, &signature, timestamp, body, NOW));
        assert!(!verify(&public, &signature[2..], timestamp, body, NOW));
        assert!(!verify(&public, "", timestamp, body, NOW));
    }

    #[test]
    fn rejects_timestamps_more_than_five_minutes_away() {
        let public = key().verifying_key();
        let body = b"{}";
        for (offset, valid) in [
            (0, true),
            (300, true),
            (-300, true),
            (301, false),
            (-301, false),
        ] {
            let timestamp = (NOW + offset).to_string();
            let signature = sign(&timestamp, body);
            assert_eq!(
                verify(&public, &signature, &timestamp, body, NOW),
                valid,
                "{offset}"
            );
        }
        let signature = sign("soon", body);
        assert!(!verify(&public, &signature, "soon", body, NOW));
    }

    #[derive(Clone, Default)]
    struct FirstWaker(Arc<Mutex<(bool, Option<Waker>)>>);

    impl FirstWaker {
        fn complete(&self) {
            let mut state = self.0.lock().unwrap();
            state.0 = true;
            if let Some(waker) = state.1.take() {
                waker.wake();
            }
        }
    }

    impl Future for FirstWaker {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            let mut state = self.0.lock().unwrap();
            if state.0 {
                return Poll::Ready(());
            }
            state.1.get_or_insert_with(|| cx.waker().clone());
            Poll::Pending
        }
    }

    #[tokio::test]
    async fn background_work_resumes_in_the_task_it_moves_to() {
        let leaf = FirstWaker::default();
        let inner = leaf.clone();
        let mut work = Relayed::new(Box::pin(inner));

        let first = Waker::noop();
        assert!(
            Pin::new(&mut work)
                .poll(&mut Context::from_waker(first))
                .is_pending()
        );

        let task = tokio::spawn(work);
        tokio::task::yield_now().await;
        leaf.complete();
        tokio::time::timeout(core::time::Duration::from_secs(5), task)
            .await
            .expect("the moved task was woken")
            .unwrap();
    }

    #[test]
    fn the_endpoint_needs_a_valid_public_key() {
        let client = Client::builder("token").build();
        let hex = hex(&key().verifying_key().to_bytes());
        assert!(client.interactions_endpoint(&hex).is_ok());
        assert_eq!(
            client.interactions_endpoint(&hex[1..]).unwrap_err(),
            InvalidPublicKey
        );
        assert_eq!(
            client.interactions_endpoint("zz").unwrap_err(),
            InvalidPublicKey
        );
    }
}
