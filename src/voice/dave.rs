use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU16,
};

use davey::{Codec, DaveSession, MediaType, ProposalsOperationType};

use crate::voice::payload::{self, Event, ProposalsOperation};

const UPGRADE_PASSTHROUGH: u32 = 10;

const MAGIC: [u8; 2] = [0xFA, 0xFA];

const SILENCE: [u8; 3] = [0xF8, 0xFF, 0xFE];

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Reply {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug)]
pub(crate) enum Handled {
    Replies(Vec<Reply>),

    Forward(Event),
}

pub(crate) struct Dave {
    user_id: u64,
    channel_id: u64,

    version: u16,
    session: Option<DaveSession>,

    external_sender: Option<Vec<u8>>,

    recognized: HashSet<u64>,

    transitions: HashMap<u16, u16>,

    frame: Vec<u8>,
}

impl core::fmt::Debug for Dave {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Dave")
            .field("user_id", &self.user_id)
            .field("channel_id", &self.channel_id)
            .field("version", &self.version)
            .field("session", &self.session.as_ref().map(DaveSession::status))
            .field("recognized", &self.recognized.len())
            .field("transitions", &self.transitions)
            .finish_non_exhaustive()
    }
}

impl Dave {
    pub(crate) fn new(user_id: u64, channel_id: u64) -> Self {
        Self {
            user_id,
            channel_id,
            version: 0,
            session: None,
            external_sender: None,
            recognized: HashSet::from([user_id]),
            transitions: HashMap::new(),
            frame: Vec::new(),
        }
    }

    pub(crate) fn start(&mut self, version: u16) -> Vec<Reply> {
        self.version = version;
        self.session = None;
        self.external_sender = None;
        self.recognized.clear();
        self.recognized.insert(self.user_id);
        self.transitions.clear();
        NonZeroU16::new(version).map_or_else(Vec::new, |version| self.init(version))
    }

    #[cfg(test)]
    pub(crate) fn ready(&self) -> bool {
        self.version == 0 || self.session.as_ref().is_some_and(DaveSession::can_encrypt)
    }

    #[cfg(test)]
    pub(crate) const fn version(&self) -> u16 {
        self.version
    }

    pub(crate) fn encrypt<'a>(&'a mut self, opus: &'a [u8]) -> Option<&'a [u8]> {
        if self.version == 0 {
            return Some(opus);
        }
        let session = self.session.as_mut()?;
        match session.encrypt_into(MediaType::AUDIO, Codec::OPUS, opus, &mut self.frame) {
            Ok(()) => Some(&self.frame),
            Err(error) => {
                tracing::trace!(%error, "not sending a frame DAVE cannot encrypt");
                None
            }
        }
    }

    pub(crate) fn decrypt<'a>(
        &mut self,
        user_id: u64,
        frame: &[u8],
        out: &'a mut Vec<u8>,
    ) -> Option<&'a [u8]> {
        out.clear();
        if let Some(session) = &mut self.session {
            if frame.ends_with(&MAGIC) {
                out.resize(frame.len(), 0);
                return match session.decrypt_into(user_id, MediaType::AUDIO, frame, out) {
                    Ok(len) => {
                        out.truncate(len);
                        Some(out)
                    }
                    Err(error) => {
                        tracing::trace!(%error, user_id, "dropping a frame DAVE cannot decrypt");
                        None
                    }
                };
            }

            let allowed = self.version == 0 || frame == SILENCE || session.can_passthrough(user_id);
            if !allowed {
                tracing::trace!(user_id, "dropping a frame that is not end-to-end encrypted");
                return None;
            }
        } else if self.version != 0 {
            return None;
        }
        out.extend_from_slice(frame);
        Some(out)
    }

    pub(crate) fn handle(&mut self, event: Event) -> Handled {
        let replies = match event {
            Event::ClientsConnect(users) => {
                self.recognized.extend(users.iter().map(|user| user.get()));
                return Handled::Forward(Event::ClientsConnect(users));
            }
            Event::ClientDisconnect(user) => {
                if user.get() != self.user_id {
                    self.recognized.remove(&user.get());
                }
                return Handled::Forward(Event::ClientDisconnect(user));
            }
            Event::DavePrepareTransition {
                transition_id,
                protocol_version,
            } => {
                if protocol_version == 0
                    && let Some(session) = &mut self.session
                {
                    session.set_passthrough_mode(true, None);
                }
                self.prepare(transition_id, protocol_version)
            }
            Event::DaveExecuteTransition { transition_id } => {
                self.execute(transition_id);
                Vec::new()
            }
            Event::DavePrepareEpoch {
                epoch,
                protocol_version,
            } => match NonZeroU16::new(protocol_version) {
                Some(version) if epoch == 1 => self.init(version),
                _ => Vec::new(),
            },
            Event::DaveExternalSender(package) => {
                if let Some(session) = &mut self.session
                    && let Err(error) = session.set_external_sender(&package)
                {
                    tracing::warn!(%error, "could not set the DAVE external sender");
                }
                self.external_sender = Some(package);
                Vec::new()
            }
            Event::DaveProposals { operation, data } => self.proposals(operation, &data),
            Event::DaveAnnounceCommit {
                transition_id,
                commit,
            } => self.commit(transition_id, &commit),
            Event::DaveWelcome {
                transition_id,
                welcome,
            } => self.welcome(transition_id, &welcome),
            other => return Handled::Forward(other),
        };
        Handled::Replies(replies)
    }

    fn init(&mut self, version: NonZeroU16) -> Vec<Reply> {
        let session = match self.session.take() {
            Some(mut session) => match session.reinit(version, self.user_id, self.channel_id, None)
            {
                Ok(()) => session,
                Err(error) => {
                    tracing::warn!(%error, "could not reinitialize the DAVE session");
                    return Vec::new();
                }
            },
            None => match DaveSession::new(version, self.user_id, self.channel_id, None) {
                Ok(mut session) => {
                    if let Some(package) = &self.external_sender
                        && let Err(error) = session.set_external_sender(package)
                    {
                        tracing::warn!(%error, "could not set the DAVE external sender");
                    }
                    session
                }
                Err(error) => {
                    tracing::warn!(%error, "could not create a DAVE session");
                    return Vec::new();
                }
            },
        };
        let session = self.session.insert(session);
        match session.create_key_package() {
            Ok(package) => {
                tracing::debug!(version, "DAVE session ready to join: key package sent");
                vec![Reply::Binary(payload::key_package(&package))]
            }
            Err(error) => {
                tracing::warn!(%error, "could not create a DAVE key package");
                Vec::new()
            }
        }
    }

    fn recognized(&self) -> Vec<u64> {
        self.recognized.iter().copied().collect()
    }

    fn proposals(&mut self, operation: ProposalsOperation, data: &[u8]) -> Vec<Reply> {
        let recognized = self.recognized();
        let Some(session) = &mut self.session else {
            return Vec::new();
        };
        let operation = match operation {
            ProposalsOperation::Append => ProposalsOperationType::APPEND,
            ProposalsOperation::Revoke => ProposalsOperationType::REVOKE,
        };
        match session.process_proposals(operation, data, Some(&recognized)) {
            Ok(Some(commit_welcome)) => {
                tracing::debug!(
                    ?operation,
                    welcome = commit_welcome.welcome.is_some(),
                    "DAVE proposals committed"
                );
                vec![Reply::Binary(payload::commit_welcome(
                    &commit_welcome.commit,
                    commit_welcome.welcome.as_deref(),
                ))]
            }
            Ok(None) => {
                tracing::debug!(?operation, "DAVE proposals left nothing to commit");
                Vec::new()
            }
            Err(error) => {
                tracing::warn!(%error, "could not process DAVE proposals");
                Vec::new()
            }
        }
    }

    fn commit(&mut self, transition_id: u16, commit: &[u8]) -> Vec<Reply> {
        let Some(session) = &mut self.session else {
            return Vec::new();
        };
        match session.process_commit(commit) {
            Ok(true) => {
                tracing::debug!(transition_id, "DAVE commit processed");
                let version = session.protocol_version().get();
                self.prepare(transition_id, version)
            }
            Ok(false) => {
                tracing::debug!(transition_id, "DAVE commit ignored");
                Vec::new()
            }
            Err(error) => {
                tracing::warn!(%error, "a DAVE commit could not be processed");
                self.invalid(transition_id)
            }
        }
    }

    fn welcome(&mut self, transition_id: u16, welcome: &[u8]) -> Vec<Reply> {
        let recognized = self.recognized();
        let Some(session) = &mut self.session else {
            return Vec::new();
        };
        match session.process_welcome(welcome, Some(&recognized)) {
            Ok(()) => {
                tracing::debug!(transition_id, "DAVE welcome processed: in the group");
                let version = session.protocol_version().get();
                self.prepare(transition_id, version)
            }
            Err(error) => {
                tracing::warn!(%error, "a DAVE welcome could not be processed");
                self.invalid(transition_id)
            }
        }
    }

    fn invalid(&mut self, transition_id: u16) -> Vec<Reply> {
        let mut replies = vec![Reply::Text(payload::invalid_commit_welcome(transition_id))];
        if let Some(version) = self.session.as_ref().map(DaveSession::protocol_version) {
            replies.extend(self.init(version));
        }
        replies
    }

    fn prepare(&mut self, transition_id: u16, version: u16) -> Vec<Reply> {
        if let Some(session) = &mut self.session
            && let Err(error) = session.prepare_transition(transition_id, version)
        {
            tracing::warn!(%error, "could not prepare a DAVE transition");
        }
        if transition_id == 0 {
            self.switch(version);
            Vec::new()
        } else {
            self.transitions.insert(transition_id, version);
            vec![Reply::Text(payload::transition_ready(transition_id))]
        }
    }

    fn execute(&mut self, transition_id: u16) {
        if let Some(session) = &mut self.session
            && let Err(error) = session.execute_transition(transition_id)
        {
            tracing::warn!(%error, "could not execute a DAVE transition");
        }
        if let Some(version) = self.transitions.remove(&transition_id) {
            self.switch(version);
        }
    }

    fn switch(&mut self, version: u16) {
        tracing::debug!(
            dave_protocol_version = version,
            can_encrypt = self.session.as_ref().is_some_and(DaveSession::can_encrypt),
            "DAVE transition executed"
        );
        let upgraded = self.version == 0 && version != 0;
        self.version = version;
        if upgraded && let Some(session) = &mut self.session {
            session.set_passthrough_mode(false, Some(UPGRADE_PASSTHROUGH));
        }
    }
}

#[cfg(test)]
pub(crate) mod mls;

#[cfg(test)]
mod tests {
    use crate::model::id::UserId;

    use super::{
        Dave, Handled, Reply,
        mls::{self, ExternalSender},
    };
    use crate::voice::payload::{Event, ProposalsOperation};

    const BOT: u64 = 1_234_123_412_341_234;
    const OTHER: u64 = 5_678_567_856_785_678;
    const CHANNEL: u64 = 42;

    const NONE: [Reply; 0] = [];

    fn replies(handled: Handled) -> Vec<Reply> {
        match handled {
            Handled::Replies(replies) => replies,
            Handled::Forward(event) => panic!("{event:?} was forwarded"),
        }
    }

    fn key_package(replies: &[Reply]) -> Vec<u8> {
        match replies {
            [Reply::Binary(message)] if message.first() == Some(&26) => message[1..].to_vec(),
            other => panic!("not a key package: {other:?}"),
        }
    }

    #[test]
    fn without_dave_frames_go_out_as_they_are() {
        let mut dave = Dave::new(BOT, CHANNEL);
        assert_eq!(dave.start(0), NONE);
        assert!(dave.ready());
        assert_eq!(dave.encrypt(b"opus"), Some(&b"opus"[..]));
    }

    #[test]
    fn the_bot_joins_through_a_welcome_and_its_frames_decrypt() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        let package = key_package(&dave.start(1));
        assert!(!dave.ready(), "no key before the group");
        assert_eq!(dave.encrypt(b"opus"), None);
        assert_eq!(
            replies(dave.handle(Event::DaveExternalSender(sender.package()))),
            NONE
        );

        let mut other = sender.member(OTHER);
        let welcome = sender.create_group(&mut other, &package, &[BOT, OTHER]);
        let Handled::Forward(_) = dave.handle(Event::ClientsConnect(vec![UserId::new(OTHER)]))
        else {
            panic!("users joining are for the call too");
        };
        assert_eq!(
            replies(dave.handle(Event::DaveWelcome {
                transition_id: 3,
                welcome
            })),
            [Reply::Text(r#"{"op":23,"d":{"transition_id":3}}"#.into())]
        );
        assert!(!dave.ready(), "not before the transition is executed");
        assert_eq!(
            replies(dave.handle(Event::DaveExecuteTransition { transition_id: 3 })),
            NONE
        );
        other.prepare_transition(3, 1).unwrap();
        other.execute_transition(3).unwrap();
        assert!(dave.ready());
        assert_eq!(dave.version(), 1);
        let frame = dave.encrypt(b"an opus frame").unwrap().to_vec();
        assert_ne!(frame, b"an opus frame");
        let decrypted = other.decrypt(BOT, davey::MediaType::AUDIO, &frame).unwrap();
        assert_eq!(decrypted, b"an opus frame");
    }

    fn in_a_group() -> (Dave, davey::DaveSession) {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        let package = key_package(&dave.start(1));
        dave.handle(Event::DaveExternalSender(sender.package()));
        let mut other = sender.member(OTHER);
        let welcome = sender.create_group(&mut other, &package, &[BOT, OTHER]);
        dave.handle(Event::ClientsConnect(vec![UserId::new(OTHER)]));
        dave.handle(Event::DaveWelcome {
            transition_id: 3,
            welcome,
        });
        dave.handle(Event::DaveExecuteTransition { transition_id: 3 });
        other.prepare_transition(3, 1).unwrap();
        other.execute_transition(3).unwrap();
        assert!(dave.ready());
        (dave, other)
    }

    #[test]
    fn frames_of_other_members_decrypt() {
        let (mut dave, mut other) = in_a_group();
        let frame = other.encrypt_opus(b"their opus frame").unwrap().to_vec();
        assert_ne!(frame, b"their opus frame");
        let mut out = Vec::new();
        assert_eq!(
            dave.decrypt(OTHER, &frame, &mut out),
            Some(&b"their opus frame"[..])
        );

        assert_eq!(
            dave.decrypt(OTHER, &[0xF8, 0xFF, 0xFE], &mut out),
            Some(&[0xF8, 0xFF, 0xFE][..])
        );

        let mut damaged = frame.clone();
        damaged[0] ^= 1;
        assert_eq!(dave.decrypt(OTHER, &damaged, &mut out), None);
        assert_eq!(dave.decrypt(BOT + 1, &frame, &mut out), None);

        assert_eq!(dave.decrypt(OTHER, b"plain opus", &mut out), None);
    }

    #[test]
    fn without_dave_received_frames_are_as_they_are() {
        let mut dave = Dave::new(BOT, CHANNEL);
        assert_eq!(dave.start(0), NONE);
        let mut out = Vec::new();
        assert_eq!(dave.decrypt(OTHER, b"opus", &mut out), Some(&b"opus"[..]));
    }

    #[test]
    fn the_bot_creates_the_group_by_committing_proposals() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        key_package(&dave.start(1));
        dave.handle(Event::DaveExternalSender(sender.package()));
        dave.handle(Event::ClientsConnect(vec![UserId::new(OTHER)]));
        let mut other = sender.member(OTHER);
        let other_package = other.create_key_package().unwrap();
        let proposals = sender.propose_adds(0, &[&other_package]);
        let [Reply::Binary(commit_welcome)] = &replies(dave.handle(Event::DaveProposals {
            operation: ProposalsOperation::Append,
            data: proposals,
        }))[..] else {
            panic!("no commit");
        };
        assert_eq!(commit_welcome[0], 28);
        let (commit, welcome) = mls::split_commit_welcome(&commit_welcome[1..]);

        assert_eq!(
            replies(dave.handle(Event::DaveAnnounceCommit {
                transition_id: 1,
                commit
            })),
            [Reply::Text(r#"{"op":23,"d":{"transition_id":1}}"#.into())]
        );
        other
            .process_welcome(&welcome.unwrap(), Some(&[BOT, OTHER]))
            .unwrap();
        other.prepare_transition(1, 1).unwrap();
        dave.handle(Event::DaveExecuteTransition { transition_id: 1 });
        other.execute_transition(1).unwrap();
        let frame = dave.encrypt(b"frame").unwrap().to_vec();
        assert_eq!(
            other.decrypt(BOT, davey::MediaType::AUDIO, &frame).unwrap(),
            b"frame"
        );
    }

    #[test]
    fn unprocessable_welcomes_are_reported_and_the_bot_starts_over() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        dave.start(1);
        dave.handle(Event::DaveExternalSender(sender.package()));
        let replies = replies(dave.handle(Event::DaveWelcome {
            transition_id: 9,
            welcome: vec![1, 2, 3],
        }));
        assert_eq!(
            replies[0],
            Reply::Text(r#"{"op":31,"d":{"transition_id":9}}"#.into())
        );
        key_package(&replies[1..]);
    }

    #[test]
    fn a_welcome_with_unknown_members_is_refused() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        let package = key_package(&dave.start(1));
        dave.handle(Event::DaveExternalSender(sender.package()));
        let mut other = sender.member(OTHER);

        let welcome = sender.create_group(&mut other, &package, &[BOT, OTHER]);
        let replies = replies(dave.handle(Event::DaveWelcome {
            transition_id: 2,
            welcome,
        }));
        assert_eq!(
            replies[0],
            Reply::Text(r#"{"op":31,"d":{"transition_id":2}}"#.into())
        );
    }

    #[test]
    fn a_new_epoch_one_restarts_with_a_key_package() {
        let mut dave = Dave::new(BOT, CHANNEL);
        assert_eq!(dave.start(0), NONE);
        key_package(&replies(dave.handle(Event::DavePrepareEpoch {
            epoch: 1,
            protocol_version: 1,
        })));
        let later = dave.handle(Event::DavePrepareEpoch {
            epoch: 2,
            protocol_version: 1,
        });
        assert_eq!(replies(later), NONE, "later epochs come with commits");
    }

    #[test]
    fn a_downgrade_sends_frames_as_they_are_after_its_execution() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        let package = key_package(&dave.start(1));
        dave.handle(Event::DaveExternalSender(sender.package()));
        dave.handle(Event::ClientsConnect(vec![UserId::new(OTHER)]));
        let mut other = sender.member(OTHER);
        let welcome = sender.create_group(&mut other, &package, &[BOT, OTHER]);
        dave.handle(Event::DaveWelcome {
            transition_id: 0,
            welcome,
        });
        assert!(dave.ready(), "transition 0 happens at once");
        assert_ne!(dave.encrypt(b"x").unwrap(), b"x");
        assert_eq!(
            replies(dave.handle(Event::DavePrepareTransition {
                transition_id: 5,
                protocol_version: 0
            })),
            [Reply::Text(r#"{"op":23,"d":{"transition_id":5}}"#.into())]
        );
        assert_ne!(
            dave.encrypt(b"x").unwrap(),
            b"x",
            "still encrypted until executed"
        );
        dave.handle(Event::DaveExecuteTransition { transition_id: 5 });
        assert_eq!(dave.version(), 0);
        assert_eq!(dave.encrypt(b"x"), Some(&b"x"[..]));
    }

    #[test]
    fn a_new_voice_session_starts_dave_over() {
        let sender = ExternalSender::new(CHANNEL);
        let mut dave = Dave::new(BOT, CHANNEL);
        let package = key_package(&dave.start(1));
        dave.handle(Event::DaveExternalSender(sender.package()));
        dave.handle(Event::ClientsConnect(vec![UserId::new(OTHER)]));
        let mut other = sender.member(OTHER);
        let welcome = sender.create_group(&mut other, &package, &[BOT, OTHER]);
        dave.handle(Event::DaveWelcome {
            transition_id: 0,
            welcome,
        });
        assert!(dave.ready());

        let new_package = key_package(&dave.start(1));
        assert_ne!(new_package, package);
        assert!(!dave.ready());
        assert_eq!(dave.recognized.len(), 1);
        assert!(dave.external_sender.is_none());
    }

    #[test]
    fn other_events_are_forwarded() {
        let mut dave = Dave::new(BOT, CHANNEL);
        assert!(matches!(
            dave.handle(Event::Resumed),
            Handled::Forward(Event::Resumed)
        ));

        dave.handle(Event::ClientDisconnect(UserId::new(BOT)));
        assert!(dave.recognized.contains(&BOT));
    }

    #[test]
    #[ignore = "a measurement; run it in release"]
    fn throughput() {
        use std::time::Instant;

        let (mut dave, mut other) = in_a_group();
        let frames: u32 = 100_000;
        for size in [160_usize, 320] {
            let opus = vec![0x7C_u8; size];
            let mut encrypt = f64::MAX;
            let mut decrypt = f64::MAX;
            let mut out = Vec::with_capacity(1500);
            for _ in 0..3 {
                let started = Instant::now();
                for _ in 0..frames {
                    std::hint::black_box(dave.encrypt(&opus));
                }
                encrypt = encrypt.min(started.elapsed().as_secs_f64());

                let sent: Vec<Vec<u8>> = (0..frames)
                    .map(|_| other.encrypt_opus(&opus).unwrap().to_vec())
                    .collect();
                let started = Instant::now();
                for frame in &sent {
                    std::hint::black_box(dave.decrypt(OTHER, frame, &mut out));
                }
                decrypt = decrypt.min(started.elapsed().as_secs_f64());
                assert_eq!(out, opus, "decrypted");
            }
            let per = |seconds: f64| seconds / f64::from(frames) * 1e9;
            println!(
                "{size}-byte Opus frame: encrypt {:.0} ns, decrypt {:.0} ns ({:.1} us per second \
                 of audio each way)",
                per(encrypt),
                per(decrypt),
                per(encrypt.max(decrypt)) * 50.0 / 1e3,
            );
        }
    }
}
