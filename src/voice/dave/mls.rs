use std::num::NonZeroU16;

use davey::{DaveSession, ProposalsOperationType};
use openmls::prelude::{
    BasicCredential, Ciphersuite, ExternalProposal, GroupEpoch, GroupId, KeyPackageIn,
    MlsMessageIn, OpenMlsProvider, ProtocolVersion, SenderExtensionIndex,
    tls_codec::{DeserializeBytes, Serialize, VLBytes},
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;

const VERSION: NonZeroU16 = NonZeroU16::MIN;

pub(crate) struct ExternalSender {
    provider: OpenMlsRustCrypto,
    group_id: GroupId,
    signer: SignatureKeyPair,
    sender: openmls::prelude::ExternalSender,
}

impl ExternalSender {
    pub(crate) fn new(channel_id: u64) -> Self {
        let ciphersuite = Ciphersuite::MLS_128_DHKEMP256_AES128GCM_SHA256_P256;
        let signer = SignatureKeyPair::new(ciphersuite.signature_algorithm()).unwrap();
        let sender = openmls::prelude::ExternalSender::new(
            signer.public().into(),
            BasicCredential::new(b"voice gateway".to_vec()).into(),
        );
        Self {
            provider: OpenMlsRustCrypto::default(),
            group_id: GroupId::from_slice(&channel_id.to_be_bytes()),
            signer,
            sender,
        }
    }

    pub(crate) fn package(&self) -> Vec<u8> {
        self.sender.tls_serialize_detached().unwrap()
    }

    pub(crate) fn member(&self, user_id: u64) -> DaveSession {
        let channel_id = u64::from_be_bytes(self.group_id.as_slice().try_into().unwrap());
        let mut session = DaveSession::new(VERSION, user_id, channel_id, None).unwrap();
        session.set_external_sender(&self.package()).unwrap();
        session
    }

    pub(crate) fn propose_adds(&self, epoch: u64, key_packages: &[&[u8]]) -> Vec<u8> {
        let mut proposals = Vec::new();
        for key_package in key_packages {
            let key_package = KeyPackageIn::tls_deserialize_exact_bytes(key_package)
                .unwrap()
                .validate(self.provider.crypto(), ProtocolVersion::Mls10)
                .unwrap();
            let proposal = ExternalProposal::new_add::<OpenMlsRustCrypto>(
                key_package,
                self.group_id.clone(),
                GroupEpoch::from(epoch),
                &self.signer,
                SenderExtensionIndex::new(0),
            )
            .unwrap();
            proposals.extend(proposal.tls_serialize_detached().unwrap());
        }
        VLBytes::new(proposals).tls_serialize_detached().unwrap()
    }

    pub(crate) fn create_group(
        &self,
        creator: &mut DaveSession,
        key_package: &[u8],
        recognized: &[u64],
    ) -> Vec<u8> {
        let commit_welcome = creator
            .process_proposals(
                ProposalsOperationType::APPEND,
                &self.propose_adds(0, &[key_package]),
                Some(recognized),
            )
            .unwrap()
            .unwrap();
        assert!(creator.process_commit(&commit_welcome.commit).unwrap());
        commit_welcome.welcome.unwrap()
    }
}

pub(crate) fn split_commit_welcome(payload: &[u8]) -> (Vec<u8>, Option<Vec<u8>>) {
    let (_, welcome) = MlsMessageIn::tls_deserialize_bytes(payload).unwrap();
    let commit = payload[..payload.len() - welcome.len()].to_vec();
    (commit, (!welcome.is_empty()).then(|| welcome.to_vec()))
}
