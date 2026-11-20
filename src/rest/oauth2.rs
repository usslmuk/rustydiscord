use core::fmt;

use crate::model::{
    application::IntegrationType,
    id::{ApplicationId, GuildId},
    oauth2::Scope,
    permissions::Permissions,
};

use crate::rest::route::write_text;

pub const AUTHORIZE_URL: &str = "https://discord.com/oauth2/authorize";

#[derive(Clone, PartialEq, Eq)]
pub struct OAuth2Credentials {
    client_id: ApplicationId,
    client_secret: String,
}

impl OAuth2Credentials {
    #[must_use]
    pub fn new(client_id: ApplicationId, client_secret: impl Into<String>) -> Self {
        Self {
            client_id,
            client_secret: client_secret.into(),
        }
    }

    #[must_use]
    pub const fn client_id(&self) -> ApplicationId {
        self.client_id
    }
}

impl fmt::Debug for OAuth2Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuth2Credentials")
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TokenTypeHint {
    AccessToken,

    RefreshToken,
}

impl TokenTypeHint {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessToken => "access_token",
            Self::RefreshToken => "refresh_token",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Prompt {
    Consent,

    None,
}

impl Prompt {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Consent => "consent",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "an authorization URL builder does nothing until it is built"]
pub struct AuthorizeUrl {
    client_id: ApplicationId,
    code: bool,
    scopes: Vec<Scope>,
    state: Option<String>,
    redirect_uri: Option<String>,
    prompt: Option<Prompt>,
    permissions: Option<Permissions>,
    guild_id: Option<GuildId>,
    disable_guild_select: bool,
    integration_type: Option<IntegrationType>,
}

impl AuthorizeUrl {
    pub const fn new(client_id: ApplicationId) -> Self {
        Self {
            client_id,
            code: false,
            scopes: Vec::new(),
            state: None,
            redirect_uri: None,
            prompt: None,
            permissions: None,
            guild_id: None,
            disable_guild_select: false,
            integration_type: None,
        }
    }

    pub fn bot(client_id: ApplicationId) -> Self {
        Self::new(client_id).scopes([Scope::Bot])
    }

    pub fn code(client_id: ApplicationId, redirect_uri: impl Into<String>) -> Self {
        let mut url = Self::new(client_id);
        url.code = true;
        url.redirect_uri(redirect_uri)
    }

    pub fn scopes(mut self, scopes: impl IntoIterator<Item = Scope>) -> Self {
        self.scopes.extend(scopes);
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub const fn prompt(mut self, prompt: Prompt) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub const fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    pub const fn guild_id(mut self, guild_id: GuildId) -> Self {
        self.guild_id = Some(guild_id);
        self
    }

    pub const fn disable_guild_select(mut self, disable: bool) -> Self {
        self.disable_guild_select = disable;
        self
    }

    pub const fn integration_type(mut self, integration_type: IntegrationType) -> Self {
        self.integration_type = Some(integration_type);
        self
    }

    #[must_use]
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for AuthorizeUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut url = String::with_capacity(128);
        url.push_str(AUTHORIZE_URL);
        url.push('?');
        if self.code {
            url.push_str("response_type=code&");
        }
        url.push_str("client_id=");

        let _ = fmt::Write::write_fmt(&mut url, format_args!("{}", self.client_id));
        if !self.scopes.is_empty() {
            url.push_str("&scope=");
            for (index, scope) in self.scopes.iter().enumerate() {
                if index > 0 {
                    url.push_str("%20");
                }
                write_text(&mut url, scope.as_str());
            }
        }
        if let Some(state) = &self.state {
            url.push_str("&state=");
            write_text(&mut url, state);
        }
        if let Some(redirect_uri) = &self.redirect_uri {
            url.push_str("&redirect_uri=");
            write_text(&mut url, redirect_uri);
        }
        if let Some(prompt) = self.prompt {
            url.push_str("&prompt=");
            url.push_str(prompt.as_str());
        }
        if let Some(permissions) = self.permissions {
            let _ = fmt::Write::write_fmt(
                &mut url,
                format_args!("&permissions={}", permissions.bits()),
            );
        }
        if let Some(guild_id) = self.guild_id {
            let _ = fmt::Write::write_fmt(&mut url, format_args!("&guild_id={guild_id}"));
        }
        if self.disable_guild_select {
            url.push_str("&disable_guild_select=true");
        }
        if let Some(integration_type) = self.integration_type {
            let _ = fmt::Write::write_fmt(
                &mut url,
                format_args!("&integration_type={}", integration_type.value()),
            );
        }
        f.write_str(&url)
    }
}

pub(crate) struct TokenForm(String);

impl TokenForm {
    pub(crate) fn new(name: &str, value: &str) -> Self {
        let mut form = Self(String::with_capacity(160));
        form.0.push_str(name);
        form.0.push('=');
        write_text(&mut form.0, value);
        form
    }

    pub(crate) fn field(mut self, name: &str, value: &str) -> Self {
        self.0.push('&');
        self.0.push_str(name);
        self.0.push('=');
        write_text(&mut self.0, value);
        self
    }

    pub(crate) fn finish(self, credentials: &OAuth2Credentials) -> String {
        let client_id = credentials.client_id.to_string();
        self.field("client_id", &client_id)
            .field("client_secret", &credentials.client_secret)
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_form_is_percent_encoded() {
        let credentials = OAuth2Credentials::new(ApplicationId::new(7), "s3cr&t=");
        let body = TokenForm::new("grant_type", "authorization_code")
            .field("code", "a b")
            .field("redirect_uri", "https://example.com/cb?x=1")
            .finish(&credentials);
        assert_eq!(
            body,
            "grant_type=authorization_code&code=a%20b\
             &redirect_uri=https%3A%2F%2Fexample.com%2Fcb%3Fx%3D1&client_id=7\
             &client_secret=s3cr%26t%3D"
        );
    }

    #[test]
    fn authorize_url_parameters() {
        let url = AuthorizeUrl::new(ApplicationId::new(1))
            .scopes([Scope::ApplicationsCommands])
            .integration_type(IntegrationType::UserInstall)
            .prompt(Prompt::None);
        assert_eq!(
            url.build(),
            "https://discord.com/oauth2/authorize?client_id=1&scope=applications.commands\
             &prompt=none&integration_type=1"
        );
        assert_eq!(
            AuthorizeUrl::new(ApplicationId::new(1)).build(),
            "https://discord.com/oauth2/authorize?client_id=1"
        );
    }

    #[test]
    fn a_bot_invite_with_every_parameter() {
        let url = AuthorizeUrl::bot(ApplicationId::new(2))
            .scopes([Scope::ApplicationsCommands])
            .permissions(Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL)
            .guild_id(GuildId::new(3))
            .disable_guild_select(true)
            .state("x y")
            .prompt(Prompt::Consent);
        let bits = (Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL).bits();
        assert_eq!(
            url.to_string(),
            format!(
                "https://discord.com/oauth2/authorize?client_id=2\
                 &scope=bot%20applications.commands&state=x%20y&prompt=consent\
                 &permissions={bits}&guild_id=3&disable_guild_select=true"
            )
        );
    }

    #[test]
    fn the_code_grant_asks_for_a_code_and_a_redirect() {
        let url = AuthorizeUrl::code(ApplicationId::new(4), "https://example.com/cb")
            .scopes([Scope::Identify])
            .redirect_uri("https://example.com/other");
        assert_eq!(
            url.build(),
            "https://discord.com/oauth2/authorize?response_type=code&client_id=4\
             &scope=identify&redirect_uri=https%3A%2F%2Fexample.com%2Fother"
        );
    }

    #[test]
    fn credentials_never_show_the_secret() {
        let credentials = OAuth2Credentials::new(ApplicationId::new(5), "very-secret");
        assert_eq!(credentials.client_id(), ApplicationId::new(5));
        let debug = format!("{credentials:?}");
        assert!(
            !debug.contains("very-secret") && debug.contains("<redacted>"),
            "{debug}"
        );
        assert_eq!(TokenTypeHint::AccessToken.as_str(), "access_token");
        assert_eq!(TokenTypeHint::RefreshToken.as_str(), "refresh_token");
    }
}
