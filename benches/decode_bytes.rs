use divan::{Bencher, black_box, counter::BytesCount};

use rustydiscord::model::gateway::{GatewayEvent, GatewayEventDeserializer};

use serde::de::DeserializeSeed;

const MESSAGE_CREATE: &str = include_str!("fixtures/message_create.json");
const READY: &str = include_str!("fixtures/ready.json");

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

#[derive(serde::Deserialize)]
struct Envelope<'a> {
    #[serde(borrow, default)]
    t: Option<&'a str>,
}

fn name_probe(bytes: &[u8]) -> Option<&str> {
    serde_json::from_slice::<Envelope<'_>>(bytes)
        .ok()
        .and_then(|envelope| envelope.t)
}

fn decode_str(json: &str) -> GatewayEvent {
    let seed = GatewayEventDeserializer::from_json(json).unwrap();
    let mut deserializer = serde_json::Deserializer::from_str(json);
    seed.deserialize(&mut deserializer).unwrap()
}

fn decode_slice(bytes: &[u8]) -> GatewayEvent {
    let text = core::str::from_utf8(bytes).unwrap();
    decode_str(text)
}

#[divan::bench]
fn name_probe_bytes(bencher: Bencher<'_, '_>) {
    let bytes = MESSAGE_CREATE.as_bytes();
    bencher
        .counter(BytesCount::of_slice(bytes))
        .bench(|| name_probe(black_box(bytes)));
}

#[divan::bench]
fn full_decode_from_str(bencher: Bencher<'_, '_>) {
    bencher
        .counter(BytesCount::of_str(MESSAGE_CREATE))
        .bench(|| decode_str(black_box(MESSAGE_CREATE)));
}

#[divan::bench]
fn full_decode_from_slice(bencher: Bencher<'_, '_>) {
    let bytes = MESSAGE_CREATE.as_bytes();
    bencher
        .counter(BytesCount::of_slice(bytes))
        .bench(|| decode_slice(black_box(bytes)));
}

#[divan::bench]
fn ready_from_slice(bencher: Bencher<'_, '_>) {
    let bytes = READY.as_bytes();
    bencher
        .counter(BytesCount::of_slice(bytes))
        .bench(|| decode_slice(black_box(bytes)));
}
