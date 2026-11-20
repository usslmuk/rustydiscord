use core::{future::Future, pin::Pin};
use std::sync::Arc;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

use super::ClusterError;

pub(crate) type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait ClusterCommand: Send + Sync + 'static {
    const NAME: &'static str;

    type Input: Serialize + DeserializeOwned + Send + 'static;

    type Output: Serialize + DeserializeOwned + Send + 'static;
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ClusterReply<T> {
    pub cluster: u32,

    pub result: Result<T, ClusterError>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub(crate) enum Failure {
    UnknownCommand,
    Failed { message: String },
    Timeout,
    NotConnected,
    Encoding { message: String },
}

impl From<Failure> for ClusterError {
    fn from(failure: Failure) -> Self {
        match failure {
            Failure::UnknownCommand => Self::UnknownCommand,
            Failure::Failed { message } => Self::CommandFailed(message),
            Failure::Timeout => Self::Timeout,
            Failure::NotConnected => Self::NotConnected,
            Failure::Encoding { message } => Self::Encoding(message),
        }
    }
}

pub(crate) type Replies = Vec<(u32, Result<Value, Failure>)>;

pub(crate) trait Commands: Send + Sync {
    fn call(&self, name: &str, input: Value) -> BoxFuture<'static, Result<Value, Failure>>;
}

pub(crate) type Handler<C> =
    Arc<dyn Fn(C, Value) -> BoxFuture<'static, Result<Value, Failure>> + Send + Sync>;

pub(crate) fn encode<T: Serialize>(value: &T) -> Result<Value, ClusterError> {
    serde_json::to_value(value).map_err(|error| ClusterError::Encoding(error.to_string()))
}

fn decode<T: DeserializeOwned>(result: Result<Value, Failure>) -> Result<T, ClusterError> {
    let value = result.map_err(ClusterError::from)?;
    serde_json::from_value(value).map_err(|error| ClusterError::Encoding(error.to_string()))
}

pub(crate) fn decode_replies<T: DeserializeOwned>(replies: Replies) -> Vec<ClusterReply<T>> {
    replies
        .into_iter()
        .map(|(cluster, result)| ClusterReply {
            cluster,
            result: decode(result),
        })
        .collect()
}

pub(crate) fn decode_one<T: DeserializeOwned>(replies: Replies) -> Result<T, ClusterError> {
    replies
        .into_iter()
        .next()
        .map_or(Err(ClusterError::NotConnected), |(_, result)| {
            decode(result)
        })
}

pub(crate) fn erase<Cmd, C, F, Fut>(handler: F) -> Handler<C>
where
    Cmd: ClusterCommand,
    C: Send + 'static,
    F: Fn(C, Cmd::Input) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Cmd::Output, crate::HandlerError>> + Send + 'static,
{
    Arc::new(move |context: C, input: Value| {
        let decoded = serde_json::from_value::<Cmd::Input>(input);
        let future = decoded.map(|input| handler(context, input));
        Box::pin(async move {
            let future = future.map_err(|error| Failure::Encoding {
                message: error.to_string(),
            })?;
            let output = future.await.map_err(|error| Failure::Failed {
                message: error.to_string(),
            })?;
            serde_json::to_value(output).map_err(|error| Failure::Encoding {
                message: error.to_string(),
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ClusterCommand, Failure, decode_one, decode_replies, erase};
    use crate::sharding::ClusterError;

    struct Double;

    impl ClusterCommand for Double {
        const NAME: &'static str = "double";
        type Input = u32;
        type Output = u64;
    }

    #[tokio::test]
    async fn handlers_decode_run_and_encode() {
        let handler = erase::<Double, (), _, _>(|(), input| async move {
            if input == 0 {
                return Err("zero".into());
            }
            Ok(u64::from(input) * 2)
        });
        assert_eq!(handler((), json!(21)).await, Ok(json!(42)));
        assert_eq!(
            handler((), json!(0)).await,
            Err(Failure::Failed {
                message: "zero".into()
            })
        );
        assert!(matches!(
            handler((), json!("not a number")).await,
            Err(Failure::Encoding { .. })
        ));
    }

    #[test]
    fn replies_keep_each_clusters_result() {
        let replies = decode_replies::<u64>(vec![
            (0, Ok(json!(2))),
            (1, Err(Failure::Timeout)),
            (2, Ok(json!("two"))),
            (3, Err(Failure::UnknownCommand)),
        ]);
        assert_eq!(
            replies
                .iter()
                .map(|reply| reply.cluster)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3]
        );
        assert_eq!(replies[0].result.as_ref().unwrap(), &2);
        assert!(matches!(replies[1].result, Err(ClusterError::Timeout)));
        assert!(matches!(replies[2].result, Err(ClusterError::Encoding(_))));
        assert!(matches!(
            replies[3].result,
            Err(ClusterError::UnknownCommand)
        ));
        assert_eq!(decode_one::<u64>(vec![(5, Ok(json!(7)))]).unwrap(), 7);
        assert!(matches!(
            decode_one::<u64>(Vec::new()),
            Err(ClusterError::NotConnected)
        ));
    }

    #[test]
    fn failures_have_a_stable_wire_format() {
        assert_eq!(
            serde_json::to_value(Failure::Failed {
                message: "m".into()
            })
            .unwrap(),
            json!({"error": "failed", "message": "m"})
        );
        assert_eq!(
            serde_json::to_value(Failure::NotConnected).unwrap(),
            json!({"error": "not_connected"})
        );
    }
}
