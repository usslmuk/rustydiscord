use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{emoji::PartialEmoji, macros::int_enum, util::Timestamp};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Poll {
    pub question: Media,

    pub answers: Vec<Answer>,

    pub expiry: Option<Timestamp>,

    pub allow_multiselect: bool,

    pub layout_type: LayoutType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Results>,
}

int_enum! {



    #[doc(alias = "PollLayoutType")]
    #[derive(Default)]
    pub enum LayoutType: u8 {

        #[default]
        Default = 1,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PollMedia")]
pub struct Media {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PollAnswer")]
pub struct Answer {
    pub answer_id: u32,

    pub poll_media: Media,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PollResults")]
pub struct Results {
    pub is_finalized: bool,

    pub answer_counts: Vec<AnswerCount>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PollAnswerCount")]
pub struct AnswerCount {
    pub id: u32,

    pub count: u32,

    pub me_voted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PollCreateRequest")]
pub struct CreatePoll {
    pub question: Media,

    #[serde(with = "create_answers")]
    pub answers: Vec<Media>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_multiselect: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_type: Option<LayoutType>,
}

impl CreatePoll {
    #[must_use]
    pub fn new(
        question: impl Into<CompactString>,
        answers: impl IntoIterator<Item = Media>,
    ) -> Self {
        Self {
            question: Media::new(question),
            answers: answers.into_iter().collect(),
            duration: None,
            allow_multiselect: None,
            layout_type: None,
        }
    }
}

impl Media {
    #[must_use]
    pub fn new(text: impl Into<CompactString>) -> Self {
        Self {
            text: Some(text.into()),
            emoji: None,
        }
    }
}

mod create_answers {
    use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};

    use super::Media;

    #[derive(Serialize)]
    struct AnswerRef<'a> {
        poll_media: &'a Media,
    }

    #[derive(Deserialize)]
    struct Answer {
        poll_media: Media,
    }

    pub(super) fn serialize<S: Serializer>(
        answers: &[Media],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(answers.len()))?;
        for poll_media in answers {
            seq.serialize_element(&AnswerRef { poll_media })?;
        }
        seq.end()
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Media>, D::Error> {
        let answers = Vec::<Answer>::deserialize(deserializer)?;
        Ok(answers
            .into_iter()
            .map(|answer| answer.poll_media)
            .collect())
    }
}
