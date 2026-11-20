use crate::model::id::UserId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetAnswerVoters {
    pub after: Option<UserId>,

    pub limit: Option<u8>,
}
