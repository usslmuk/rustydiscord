use compact_str::CompactString;
use serde::Serialize;

use crate::application_identity_profile::ProfileData;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct UpdateApplicationIdentityProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProfileData>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetApplicationIdentitiesByExternalId {
    pub provider_id: Option<CompactString>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct DeleteApplicationIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<CompactString>,
}
