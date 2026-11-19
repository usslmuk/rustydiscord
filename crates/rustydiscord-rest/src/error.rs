use core::{fmt, time::Duration};

use http::StatusCode;
use rustydiscord_model::rest::ErrorCode;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct Error {
    kind: ErrorKind,
    #[source]
    source: Option<BoxError>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }

    pub(crate) fn with_source(kind: ErrorKind, source: impl Into<BoxError>) -> Self {
        Self {
            kind,
            source: Some(source.into()),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[must_use]
    pub fn into_parts(self) -> (ErrorKind, Option<Box<dyn std::error::Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        match &self.kind {
            ErrorKind::Response { status, .. } => Some(*status),
            _ => None,
        }
    }

    #[must_use]
    pub fn api_error(&self) -> Option<&ApiError> {
        match &self.kind {
            ErrorKind::Response { error, .. } => error.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorKind {
    #[error("failed to build the request")]
    BuildRequest,

    #[error("failed to send the request or receive the response")]
    Transport,

    #[error("the request timed out")]
    Timeout,

    #[error("Discord responded with {status}{}", error.as_ref().map_or_default(|error| format!(": {error}")))]
    Response {
        status: StatusCode,

        error: Option<Box<ApiError>>,
    },

    #[error("failed to deserialize the response body")]
    Deserialize,

    #[error("the guild is not indexed for search yet; retry after {retry_after:?}")]
    NotYetIndexed {
        retry_after: Duration,

        documents_indexed: u64,
    },

    #[error("refused to send: {count} invalid requests in the last 10 minutes (limit {limit})")]
    InvalidRequestLimit { count: u32, limit: u32 },
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ApiError {
    pub code: ErrorCode,

    pub message: String,

    pub field_errors: Vec<FieldError>,

    pub errors: Option<Value>,
}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            code: ErrorCode,
            message: String,
            #[serde(default)]
            errors: Option<Value>,
        }

        let raw = Raw::deserialize(deserializer)?;
        let mut field_errors = Vec::new();
        if let Some(errors) = &raw.errors {
            flatten_errors(errors, &mut String::new(), &mut field_errors);
        }
        Ok(Self {
            code: raw.code,
            message: raw.message,
            field_errors,
            errors: raw.errors,
        })
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message, self.code.value())?;
        for error in &self.field_errors {
            write!(f, "\n{error}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct FieldError {
    pub path: String,

    pub code: String,

    pub message: String,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.path.is_empty(), self.code.is_empty()) {
            (false, false) => write!(f, "{}[{}]: {}", self.path, self.code, self.message),
            (false, true) => write!(f, "{}: {}", self.path, self.message),
            (true, false) => write!(f, "{}: {}", self.code, self.message),
            (true, true) => f.write_str(&self.message),
        }
    }
}

fn flatten_errors(value: &Value, path: &mut String, out: &mut Vec<FieldError>) {
    match value {
        Value::Object(object) => {
            if let (Some(Value::String(code)), Some(Value::String(message))) =
                (object.get("code"), object.get("message"))
            {
                out.push(FieldError {
                    path: path.clone(),
                    code: code.clone(),
                    message: message.clone(),
                });
                return;
            }
            for (key, child) in object {
                let length = path.len();
                if !key.starts_with('_') {
                    if path.is_empty() {
                        path.push_str(key);
                    } else if key.bytes().all(|byte| byte.is_ascii_digit()) {
                        path.push('[');
                        path.push_str(key);
                        path.push(']');
                    } else {
                        path.push('.');
                        path.push_str(key);
                    }
                }
                flatten_errors(child, path, out);
                path.truncate(length);
            }
        }
        Value::Array(items) => {
            for item in items {
                flatten_errors(item, path, out);
            }
        }
        Value::String(message) => out.push(FieldError {
            path: path.clone(),
            code: String::new(),
            message: message.clone(),
        }),
        _ => {}
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct RateLimitedBody {
    pub(crate) retry_after: f64,

    #[serde(default)]
    pub(crate) global: bool,
}

impl RateLimitedBody {
    pub(crate) fn retry_after(&self) -> Duration {
        seconds(self.retry_after)
    }
}

pub(crate) fn not_yet_indexed(body: &[u8]) -> Error {
    #[derive(Deserialize, Default)]
    struct NotIndexed {
        #[serde(default)]
        retry_after: f64,
        #[serde(default)]
        documents_indexed: u64,
    }

    let body: NotIndexed = serde_json::from_slice(body).unwrap_or_default();
    Error::new(ErrorKind::NotYetIndexed {
        retry_after: seconds(body.retry_after),
        documents_indexed: body.documents_indexed,
    })
}

pub(crate) fn seconds(value: f64) -> Duration {
    if value.is_finite() && value > 0.0 {
        Duration::from_secs_f64(value.min(3_600.0))
    } else {
        Duration::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let error = Error::new(ErrorKind::Response {
            status: StatusCode::FORBIDDEN,
            error: Some(Box::new(ApiError {
                code: ErrorCode::MissingPermissions,
                message: "Missing Permissions".into(),
                field_errors: Vec::new(),
                errors: None,
            })),
        });
        assert_eq!(
            error.to_string(),
            "Discord responded with 403 Forbidden: Missing Permissions (code 50013)"
        );
        assert_eq!(error.status(), Some(StatusCode::FORBIDDEN));
        assert_eq!(
            error.api_error().unwrap().code,
            ErrorCode::MissingPermissions
        );
    }

    fn parse(json: &str) -> ApiError {
        serde_json::from_str(json).unwrap()
    }

    fn lines(error: &ApiError) -> Vec<String> {
        error.field_errors.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn field_errors_of_the_docs_examples() {
        let array = parse(
            r#"{"code": 50035, "errors": {"activities": {"0": {
                "platform": {"_errors": [{"code": "BASE_TYPE_CHOICES",
                    "message": "Value must be one of ('desktop', 'android', 'ios')."}]},
                "type": {"_errors": [{"code": "BASE_TYPE_CHOICES",
                    "message": "Value must be one of (0, 1, 2, 3, 4, 5)."}]}}}},
                "message": "Invalid Form Body"}"#,
        );
        assert_eq!(array.code, ErrorCode::InvalidFormBodyOrContentType);
        assert_eq!(
            lines(&array),
            [
                "activities[0].platform[BASE_TYPE_CHOICES]: Value must be one of ('desktop', 'android', 'ios').",
                "activities[0].type[BASE_TYPE_CHOICES]: Value must be one of (0, 1, 2, 3, 4, 5).",
            ]
        );
        assert!(array.errors.is_some());

        let object = parse(
            r#"{"code": 50035, "errors": {"access_token": {"_errors": [
                {"code": "BASE_TYPE_REQUIRED", "message": "This field is required"}]}},
                "message": "Invalid Form Body"}"#,
        );
        assert_eq!(
            object.to_string(),
            "Invalid Form Body (code 50035)\naccess_token[BASE_TYPE_REQUIRED]: This field is required"
        );

        let request = parse(
            r#"{"code": 50035, "message": "Invalid Form Body", "errors": {"_errors": [
                {"code": "APPLICATION_COMMAND_TOO_LARGE",
                 "message": "Command exceeds maximum size (8000)"}]}}"#,
        );
        assert_eq!(request.field_errors[0].path, "");
        assert_eq!(
            lines(&request),
            ["APPLICATION_COMMAND_TOO_LARGE: Command exceeds maximum size (8000)"]
        );
    }

    #[test]
    fn unusual_error_shapes() {
        let error = parse(
            r#"{"code": 123456, "message": "Something", "errors": {
                "0": {"code": {"_errors": [{"code": "NUMBER_TYPE_MAX", "message": "Too big"}]}},
                "name": "Must be set",
                "other": 5}}"#,
        );
        assert_eq!(error.code, ErrorCode::Unknown(123_456));
        assert_eq!(
            lines(&error),
            ["0.code[NUMBER_TYPE_MAX]: Too big", "name: Must be set"]
        );
        let plain = parse(r#"{"code": 0, "message": "404: Not Found"}"#);
        assert_eq!(plain.code, ErrorCode::GeneralError);
        assert!(plain.field_errors.is_empty() && plain.errors.is_none());
        assert_eq!(plain.to_string(), "404: Not Found (code 0)");
    }

    #[test]
    fn size() {
        assert!(size_of::<Error>() <= 48, "{}", size_of::<Error>());
    }

    #[test]
    fn seconds_is_robust() {
        assert_eq!(seconds(f64::NAN), Duration::ZERO);
        assert_eq!(seconds(-1.0), Duration::ZERO);
        assert_eq!(seconds(0.25), Duration::from_millis(250));
        assert_eq!(seconds(1e12), Duration::from_secs(3_600));
    }
}
