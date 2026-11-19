#![cfg_attr(docsrs, feature(doc_cfg))]

mod call;
mod codec;
mod connection;
mod crypto;
mod dave;
mod error;
mod gateway;
mod groups;
mod input;
mod manager;
mod ogg;
mod opus;
mod payload;
mod player;
mod queue;
mod receive;
mod report;
mod tls;
mod track;
mod udp;

pub use crate::{
    call::{Call, CallEvent, CallEvents, DisconnectReason, WaitError},
    error::{InputError, JoinError, TrackError},
    gateway::VoiceGateway,
    groups::Group,
    input::Input,
    input::{ProbedKind, probe},
    manager::{JoinOptions, VoiceManager},
    player::{AudioPlayer, AudioPlayerStatus, AudioResource, create_audio_resource},
    queue::TrackQueue,
    receive::{EndBehavior, ReceiveOptions, VoiceData, VoicePacket, VoiceStream},
    report::{DependencyReport, generate_dependency_report},
    track::{Loop, PlayState, TrackEnd, TrackEvent, TrackEvents, TrackHandle, TrackInfo},
};
