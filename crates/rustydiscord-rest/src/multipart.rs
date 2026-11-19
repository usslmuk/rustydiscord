use core::fmt::Write as _;
use std::hash::{BuildHasher, RandomState};

use bytes::Bytes;
use http::HeaderValue;
use rustydiscord_model::rest::FileUpload;

use crate::transport::Body;

const OCTET_STREAM: &str = "application/octet-stream";

#[derive(Clone, Debug, Default)]
pub(crate) struct PreparedBody {
    chunks: Vec<Bytes>,
    content_type: Option<HeaderValue>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Form<'a> {
    pub(crate) files: &'a [FileUpload],

    pub(crate) file: Option<(&'a str, &'a FileUpload)>,

    pub(crate) text: &'a [(&'a str, &'a str)],
}

impl Form<'_> {
    const fn is_empty(&self) -> bool {
        self.files.is_empty() && self.file.is_none() && self.text.is_empty()
    }
}

impl PreparedBody {
    pub(crate) fn new(json: Option<Bytes>, form: &Form<'_>) -> Self {
        if form.is_empty() {
            return Self {
                content_type: json
                    .is_some()
                    .then(|| HeaderValue::from_static("application/json")),
                chunks: json.into_iter().collect(),
            };
        }
        multipart(json, form, &boundary())
    }

    pub(crate) fn urlencoded(body: Bytes) -> Self {
        Self {
            chunks: vec![body],
            content_type: Some(HeaderValue::from_static(
                "application/x-www-form-urlencoded",
            )),
        }
    }

    pub(crate) const fn content_type(&self) -> Option<&HeaderValue> {
        self.content_type.as_ref()
    }

    pub(crate) fn body(&self) -> Body {
        Body::new(self.chunks.clone())
    }
}

fn boundary() -> String {
    let random = RandomState::new();
    format!(
        "discord-rs-{:016x}{:016x}",
        random.hash_one(0_u8),
        random.hash_one(1_u8)
    )
}

#[derive(Clone, Copy)]
enum FieldName<'a> {
    Indexed(usize),

    Named(&'a str),
}

fn multipart(json: Option<Bytes>, form: &Form<'_>, boundary: &str) -> PreparedBody {
    let parts = form
        .files
        .iter()
        .enumerate()
        .map(|(index, file)| (FieldName::Indexed(index), file))
        .chain(form.file.map(|(name, file)| (FieldName::Named(name), file)));
    let mut chunks = Vec::with_capacity(form.files.len() * 2 + 5);

    let mut text = String::with_capacity(256);
    for (index, (field, file)) in parts.enumerate() {
        if index > 0 {
            text.push_str("\r\n");
        }

        let _ = write!(
            text,
            "--{boundary}\r\nContent-Disposition: form-data; name=\""
        );
        match field {
            FieldName::Indexed(n) => {
                let _ = write!(text, "files[{n}]");
            }
            FieldName::Named(name) => write_quoted(&mut text, name),
        }
        text.push_str("\"; filename=\"");
        write_quoted(&mut text, &file.name);
        text.push_str("\"\r\nContent-Type: ");
        text.push_str(content_type(file));
        text.push_str("\r\n\r\n");
        chunks.push(Bytes::from(core::mem::take(&mut text)));
        chunks.push(file.data.clone());
    }
    text.push_str("\r\n");
    for (name, value) in form.text {
        let _ = write!(
            text,
            "--{boundary}\r\nContent-Disposition: form-data; name=\""
        );
        write_quoted(&mut text, name);
        text.push_str("\"\r\n\r\n");
        text.push_str(value);
        text.push_str("\r\n");
    }
    if let Some(json) = json {
        let _ = write!(
            text,
            "--{boundary}\r\nContent-Disposition: form-data; name=\"payload_json\"\r\n\
             Content-Type: application/json\r\n\r\n"
        );
        chunks.push(Bytes::from(core::mem::take(&mut text)));
        chunks.push(json);
        text.push_str("\r\n");
    }
    let _ = write!(text, "--{boundary}--\r\n");
    chunks.push(Bytes::from(text));

    let content_type = HeaderValue::try_from(format!("multipart/form-data; boundary={boundary}"))
        .unwrap_or_else(|_| HeaderValue::from_static("multipart/form-data"));
    PreparedBody {
        chunks,
        content_type: Some(content_type),
    }
}

fn write_quoted(out: &mut String, name: &str) {
    for character in name.chars() {
        match character {
            '"' => out.push_str("%22"),
            '\r' => out.push_str("%0D"),
            '\n' => out.push_str("%0A"),
            _ => out.push(character),
        }
    }
}

fn content_type(file: &FileUpload) -> &str {
    match &file.content_type {
        Some(given)
            if !given.is_empty() && given.bytes().all(|byte| (b' '..=b'~').contains(&byte)) =>
        {
            given
        }
        _ => guess_content_type(&file.name),
    }
}

fn guess_content_type(name: &str) -> &'static str {
    let Some((_, extension)) = name.rsplit_once('.') else {
        return OCTET_STREAM;
    };
    let mut lower = [0_u8; 4];
    let Some(target) = lower.get_mut(..extension.len()) else {
        return OCTET_STREAM;
    };
    target.copy_from_slice(extension.as_bytes());
    target.make_ascii_lowercase();
    match &*target {
        b"png" => "image/png",
        b"apng" => "image/apng",
        b"jpg" | b"jpeg" | b"jfif" => "image/jpeg",
        b"gif" => "image/gif",
        b"webp" => "image/webp",
        b"avif" => "image/avif",
        b"heic" => "image/heic",
        b"heif" => "image/heif",
        b"bmp" => "image/bmp",
        b"svg" => "image/svg+xml",
        b"tif" | b"tiff" => "image/tiff",
        b"ico" => "image/x-icon",

        b"mp4" | b"m4v" => "video/mp4",
        b"webm" => "video/webm",
        b"mov" => "video/quicktime",
        b"mkv" => "video/x-matroska",
        b"avi" => "video/x-msvideo",

        b"mp3" => "audio/mpeg",
        b"ogg" | b"oga" | b"opus" => "audio/ogg",
        b"wav" => "audio/wav",
        b"flac" => "audio/flac",
        b"m4a" => "audio/mp4",
        b"aac" => "audio/aac",

        b"txt" | b"log" => "text/plain; charset=utf-8",
        b"md" => "text/markdown; charset=utf-8",
        b"csv" => "text/csv; charset=utf-8",
        b"htm" | b"html" => "text/html; charset=utf-8",
        b"css" => "text/css; charset=utf-8",
        b"js" | b"mjs" => "text/javascript; charset=utf-8",
        b"json" => "application/json",
        b"xml" => "application/xml",
        b"yml" | b"yaml" => "application/yaml",
        b"toml" => "application/toml",

        b"pdf" => "application/pdf",
        b"zip" => "application/zip",
        b"gz" => "application/gzip",
        b"tar" => "application/x-tar",
        b"wasm" => "application/wasm",
        _ => OCTET_STREAM,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(body: &PreparedBody) -> (Vec<Bytes>, Vec<u8>) {
        let body = body.body();
        let frames: Vec<Bytes> = body
            .chunks()
            .iter()
            .filter(|chunk| !chunk.is_empty())
            .cloned()
            .collect();
        (frames, body.into_bytes().to_vec())
    }

    #[tokio::test]
    async fn json_only() {
        let body = PreparedBody::new(Some(Bytes::from_static(b"{}")), &Form::default());
        assert_eq!(body.content_type().unwrap(), "application/json");
        assert_eq!(body.body().len(), 2);
        assert_eq!(collect(&body).1, b"{}");

        let empty = PreparedBody::new(None, &Form::default());
        assert!(empty.content_type().is_none());
        assert!(empty.body().is_empty());
        assert_eq!(collect(&empty).0.len(), 0);
    }

    #[tokio::test]
    async fn multipart_layout() {
        let image = FileUpload::new("ferris.png", Bytes::from_static(b"PNGDATA"));
        let mut text = FileUpload::new("a\"b\r\n.txt", "hello");
        text.content_type = Some("text/x-custom".into());
        let files = [image, text];
        let body = multipart(
            Some(Bytes::from_static(b"{\"content\":\"hi\"}")),
            &Form {
                files: &files,
                ..Form::default()
            },
            "XYZ",
        );
        assert_eq!(
            body.content_type().unwrap(),
            "multipart/form-data; boundary=XYZ"
        );
        let (frames, all) = collect(&body);
        assert_eq!(
            String::from_utf8(all).unwrap(),
            "--XYZ\r\n\
             Content-Disposition: form-data; name=\"files[0]\"; filename=\"ferris.png\"\r\n\
             Content-Type: image/png\r\n\r\n\
             PNGDATA\r\n\
             --XYZ\r\n\
             Content-Disposition: form-data; name=\"files[1]\"; filename=\"a%22b%0D%0A.txt\"\r\n\
             Content-Type: text/x-custom\r\n\r\n\
             hello\r\n\
             --XYZ\r\n\
             Content-Disposition: form-data; name=\"payload_json\"\r\n\
             Content-Type: application/json\r\n\r\n\
             {\"content\":\"hi\"}\r\n\
             --XYZ--\r\n"
        );

        assert_eq!(frames[1].as_ptr(), files[0].data.as_ptr());
        assert_eq!(frames[3].as_ptr(), files[1].data.as_ptr());

        assert_eq!(collect(&body).0.len(), frames.len());
        let length: usize = frames.iter().map(Bytes::len).sum();
        assert_eq!(body.body().len(), length);
    }

    #[tokio::test]
    async fn multipart_without_json_and_with_empty_file() {
        let files = [FileUpload::new("empty.bin", Bytes::new())];
        let body = multipart(
            None,
            &Form {
                files: &files,
                ..Form::default()
            },
            "B",
        );
        assert_eq!(
            String::from_utf8(collect(&body).1).unwrap(),
            "--B\r\nContent-Disposition: form-data; name=\"files[0]\"; filename=\"empty.bin\"\r\n\
             Content-Type: application/octet-stream\r\n\r\n\r\n--B--\r\n"
        );
    }

    #[tokio::test]
    async fn named_form_file() {
        let csv = FileUpload::new("users.csv", "1\n2\n");
        let body = PreparedBody::new(
            Some(Bytes::from_static(b"{}")),
            &Form {
                file: Some(("target_users_file", &csv)),
                ..Form::default()
            },
        );
        let content_type = body.content_type().unwrap().to_str().unwrap();
        let boundary = content_type
            .strip_prefix("multipart/form-data; boundary=")
            .unwrap();
        assert_eq!(
            String::from_utf8(collect(&body).1).unwrap(),
            format!(
                "--{boundary}\r\n\
                 Content-Disposition: form-data; name=\"target_users_file\"; filename=\"users.csv\"\r\n\
                 Content-Type: text/csv; charset=utf-8\r\n\r\n\
                 1\n2\n\r\n\
                 --{boundary}\r\n\
                 Content-Disposition: form-data; name=\"payload_json\"\r\n\
                 Content-Type: application/json\r\n\r\n\
                 {{}}\r\n\
                 --{boundary}--\r\n"
            )
        );
    }

    #[test]
    fn boundaries_are_random() {
        let a = boundary();
        let b = boundary();
        assert_ne!(a, b);
        assert_eq!(a.len(), "discord-rs-".len() + 32);
        assert!(
            a.bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        );
    }

    #[test]
    fn content_types() {
        for (name, expected) in [
            ("ferris.PNG", "image/png"),
            ("photo.jpeg", "image/jpeg"),
            ("voice-message.ogg", "audio/ogg"),
            ("clip.webm", "video/webm"),
            ("notes.txt", "text/plain; charset=utf-8"),
            ("archive.tar.gz", "application/gzip"),
            ("no-extension", OCTET_STREAM),
            ("trailing.", OCTET_STREAM),
            ("long.extension", OCTET_STREAM),
            ("unknown.xyz", OCTET_STREAM),
            ("ünïcode.png", "image/png"),
            ("dot.ñ", OCTET_STREAM),
        ] {
            assert_eq!(guess_content_type(name), expected, "{name}");
        }

        let mut file = FileUpload::new("data.png", "x");
        file.content_type = Some("image/x-special".into());
        assert_eq!(content_type(&file), "image/x-special");
        file.content_type = Some("text/plain\r\nX-Evil: 1".into());
        assert_eq!(content_type(&file), "image/png");
        file.content_type = Some(String::new().into());
        assert_eq!(content_type(&file), "image/png");
    }
}
