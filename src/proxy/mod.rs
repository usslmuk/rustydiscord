#![cfg_attr(docsrs, feature(doc_cfg))]

const SLASH: char = '/';
const AUTHORIZATION: &str = "authorization";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyConfig {
    pub target: String,
    pub redact_token: bool,
}

impl ProxyConfig {
    #[must_use]
    pub fn new(target: String, redact_token: bool) -> Self {
        Self {
            target,
            redact_token,
        }
    }

    #[must_use]
    pub fn proxy_url_for(&self, path: &str) -> String {
        let base = self.target.trim_end_matches(SLASH);
        let tail = path.trim_start_matches(SLASH);
        let mut url = String::with_capacity(base.len() + tail.len() + 1);
        url.push_str(base);
        url.push(SLASH);
        url.push_str(tail);
        url
    }

    #[must_use]
    pub fn forward_headers(&self, headers: &[(String, String)]) -> Vec<(String, String)> {
        if self.redact_token {
            headers
                .iter()
                .filter(|(name, _)| !is_authorization(name))
                .cloned()
                .collect()
        } else {
            headers.to_vec()
        }
    }

    #[must_use]
    pub fn forward(
        &self,
        path: &str,
        headers: &[(String, String)],
    ) -> (String, Vec<(String, String)>) {
        (self.proxy_url_for(path), self.forward_headers(headers))
    }
}

fn is_authorization(name: &str) -> bool {
    name.eq_ignore_ascii_case(AUTHORIZATION)
}

#[cfg(test)]
mod tests {
    use super::ProxyConfig;

    fn target() -> String {
        ["https:", "", "proxy.local"].join("/")
    }

    fn headers() -> Vec<(String, String)> {
        vec![
            (String::from("Authorization"), String::from("Bot secret")),
            (
                String::from("Content-Type"),
                String::from("application/json"),
            ),
        ]
    }

    #[test]
    fn joins_target_and_path_with_single_slash() {
        let config = ProxyConfig::new(target(), true);
        assert_eq!(
            config.proxy_url_for("api/v1/users"),
            target() + "/api/v1/users"
        );
        assert_eq!(
            config.proxy_url_for("/api/v1/users"),
            target() + "/api/v1/users"
        );
        let trailing = ProxyConfig::new(target() + "/", true);
        assert_eq!(
            trailing.proxy_url_for("/api/v1/users"),
            target() + "/api/v1/users"
        );
    }

    #[test]
    fn redacting_strips_authorization_headers() {
        let config = ProxyConfig::new(target(), true);
        let forwarded = config.forward_headers(&headers());
        assert_eq!(forwarded.len(), 1);
        assert_eq!(forwarded[0].0, "Content-Type");
        let (url, kept) = config.forward("api", &headers());
        assert_eq!(url, target() + "/api");
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn disabled_redaction_keeps_every_header() {
        let config = ProxyConfig::new(target(), false);
        let forwarded = config.forward_headers(&headers());
        assert_eq!(forwarded, headers());
    }

    #[test]
    fn authorization_match_ignores_case() {
        let config = ProxyConfig::new(target(), true);
        let mixed = vec![(String::from("authorization"), String::from("Bot secret"))];
        assert_eq!(config.forward_headers(&mixed), []);
    }
}
