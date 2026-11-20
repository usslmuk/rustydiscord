use crate::model::{
    application::Application,
    oauth2::{AccessTokenResponse, CurrentAuthorizationInformation, Scope},
};

use crate::rest::client::boxed;
use crate::rest::{
    Error, Request, Rest, Route,
    oauth2::{OAuth2Credentials, TokenForm, TokenTypeHint},
};

impl Rest {
    pub fn get_current_bot_application_information(
        &self,
    ) -> impl Future<Output = Result<Application, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentBotApplicationInformation))
                .await
        })
    }

    pub fn get_current_authorization_information(
        &self,
    ) -> impl Future<Output = Result<CurrentAuthorizationInformation, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentAuthorizationInformation))
                .await
        })
    }

    pub fn exchange_code<'a>(
        &'a self,
        credentials: &'a OAuth2Credentials,
        code: &'a str,
        redirect_uri: &'a str,
    ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a {
        boxed(async move {
            let form = TokenForm::new("grant_type", "authorization_code")
                .field("code", code)
                .field("redirect_uri", redirect_uri)
                .finish(credentials);
            self.call(Request::new(Route::OAuth2TokenExchange).form_urlencoded(form))
                .await
        })
    }

    pub fn refresh_access_token<'a>(
        &'a self,
        credentials: &'a OAuth2Credentials,
        refresh_token: &'a str,
    ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a {
        boxed(async move {
            let form = TokenForm::new("grant_type", "refresh_token")
                .field("refresh_token", refresh_token)
                .finish(credentials);
            self.call(Request::new(Route::OAuth2TokenExchange).form_urlencoded(form))
                .await
        })
    }

    pub fn client_credentials_grant<'a>(
        &'a self,
        credentials: &'a OAuth2Credentials,
        scopes: &'a [Scope],
    ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a {
        boxed(async move {
            let scope = scopes
                .iter()
                .map(Scope::as_str)
                .collect::<Vec<_>>()
                .join(" ");
            let form = TokenForm::new("grant_type", "client_credentials")
                .field("scope", &scope)
                .finish(credentials);
            self.call(Request::new(Route::OAuth2TokenExchange).form_urlencoded(form))
                .await
        })
    }

    pub fn revoke_token<'a>(
        &'a self,
        credentials: &'a OAuth2Credentials,
        token: &'a str,
        hint: Option<TokenTypeHint>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            let mut form = TokenForm::new("token", token);
            if let Some(hint) = hint {
                form = form.field("token_type_hint", hint.as_str());
            }
            self.request_ignoring_body(
                Request::new(Route::OAuth2TokenRevocation)
                    .form_urlencoded(form.finish(credentials)),
            )
            .await
        })
    }
}
