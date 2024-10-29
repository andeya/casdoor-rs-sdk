mod models;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
pub use models::*;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenUrl};
pub use oauth2::{basic::BasicTokenType, AccessToken, RefreshToken, Scope, TokenResponse, TokenType};

use crate::{Method, QueryArgs, Sdk, SdkResult, NONE_BODY};
impl Sdk {
    pub fn authn(&self) -> AuthSdk {
        AuthSdk { sdk: self.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct AuthSdk {
    sdk: Sdk,
}

impl AuthSdk {
    fn client_id(&self) -> ClientId {
        ClientId::new(self.sdk.client_id().clone())
    }
    fn client_secret(&self) -> Option<ClientSecret> {
        Some(ClientSecret::new(self.sdk.client_secret().clone()))
    }
    fn auth_url(&self, url_path: &str) -> Result<AuthUrl, oauth2::url::ParseError> {
        AuthUrl::new(self.sdk.endpoint().clone() + url_path)
    }
    fn token_url(&self, url_path: &str) -> Result<Option<TokenUrl>, oauth2::url::ParseError> {
        Ok(Some(TokenUrl::new(self.sdk.endpoint().clone() + url_path)?))
    }

    /// Gets the pivotal and necessary secret to interact with the Casdoor server
    pub async fn get_oauth_token(&self, code: String) -> SdkResult<impl TokenResponse<BasicTokenType>> {
        Ok(BasicClient::new(
            self.client_id(),
            self.client_secret(),
            self.auth_url("/api/login/oauth/authorize")?,
            self.token_url("/api/login/oauth/access_token")?,
        )
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?)
    }

    /// Refreshes the OAuth token
    pub async fn refresh_oauth_token(&self, refresh_token: String) -> SdkResult<impl TokenResponse<BasicTokenType>> {
        Ok(BasicClient::new(
            self.client_id(),
            self.client_secret(),
            self.auth_url("/api/login/oauth/authorize")?,
            self.token_url("/api/login/oauth/refresh_token")?,
        )
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await?)
    }

    pub fn parse_jwt_token(&self, token: &str) -> SdkResult<Claims> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.sdk.client_id()]);

        let td: TokenData<Claims> = jsonwebtoken::decode(token, &DecodingKey::from_rsa_pem(self.sdk.certificate().as_bytes())?, &validation)?;
        Ok(td.claims)
    }

    pub fn get_signin_url(&self, redirect_url: String) -> String {
        let scope = "read";
        let state = self.sdk.app_name().clone().unwrap_or_default();
        format!(
            "{}/login/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope={scope}&state={state}",
            self.sdk.endpoint(),
            self.sdk.client_id(),
            urlencoding::encode(&redirect_url).into_owned(),
        )
    }

    pub fn get_signup_url(&self, redirect_url: String) -> String {
        redirect_url.replace("/login/oauth/authorize", "/signup/oauth/authorize")
    }

    pub fn get_signup_url_enable_password(&self) -> String {
        format!("{}/signup/{}", self.sdk.endpoint(), self.sdk.app_name().clone().unwrap_or_default())
    }

    pub fn get_user_profile_url(&self, uname: String, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!("{}/users/{}/{uname}{param}", self.sdk.endpoint(), self.sdk.org_name())
    }

    pub fn get_my_profile_url(&self, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!("{}/account{}", self.sdk.endpoint(), param)
    }

    pub async fn get_sessions(&self, query_args: QueryArgs) -> SdkResult<Vec<Session>> {
        self.sdk
            .request_data(Method::GET, self.sdk.get_url_path("get-sessions", true, query_args)?, NONE_BODY)
            .await?
            .into_data_default()
    }

    pub async fn get_session(&self, session_pk_id: &str) -> SdkResult<Session> {
        self.sdk
            .request_data(
                Method::GET,
                self.sdk.get_url_path("get-session", true, ("sessionPkId", session_pk_id))?,
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn is_session_duplicated(&self, session_pk_id: &str, session_id: &str) -> SdkResult<bool> {
        self.sdk
            .request_data(
                Method::GET,
                self.sdk.get_url_path(
                    "is-session-duplicated",
                    true,
                    &[("sessionPkId", session_pk_id), ("sessionId", session_id)],
                )?,
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }
}
