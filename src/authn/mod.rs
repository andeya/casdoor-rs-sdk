mod models;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
pub use models::*;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenUrl};
pub use oauth2::{basic::BasicTokenType, AccessToken, RefreshToken, Scope, TokenResponse, TokenType};

use crate::{Method, QueryArgs, QueryResult, Sdk, SdkResult, NO_BODY};
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

    pub fn parse_jwt_token_rs256(&self, token: &str) -> SdkResult<Claims> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.sdk.client_id()]);

        let td: TokenData<Claims> = jsonwebtoken::decode(token, &DecodingKey::from_rsa_pem(self.sdk.certificate().as_bytes())?, &validation)?;
        Ok(td.claims)
    }

    pub fn parse_jwt_token_rs512(&self, token: &str) -> SdkResult<Claims> {
        let mut validation: Validation = Validation::new(Algorithm::RS512);
        validation.set_audience(&[self.sdk.client_id()]);

        let td: TokenData<Claims> = jsonwebtoken::decode(token, &DecodingKey::from_rsa_pem(self.sdk.certificate().as_bytes())?, &validation)?;

        Ok(td.claims)
    }

    pub fn parse_jwt_token_es256(&self, token: &str) -> SdkResult<Claims> {
        let mut validation: Validation = Validation::new(Algorithm::ES256);
        validation.set_audience(&[self.sdk.client_id()]);

        let decode_key = &DecodingKey::from_ec_pem(self.sdk.certificate().as_bytes())?;

        let token_data: TokenData<Claims> = jsonwebtoken::decode(&token, &decode_key, &validation)?;

        Ok(token_data.claims)
    }

    pub fn parse_jwt_token_es384(&self, token: &str) -> SdkResult<Claims> {
        let mut validation: Validation = Validation::new(Algorithm::ES384);
        validation.set_audience(&[self.sdk.client_id()]);

        let decode_key = &DecodingKey::from_ec_pem(self.sdk.certificate().as_bytes())?;

        let token_data: TokenData<Claims> = jsonwebtoken::decode(&token, &decode_key, &validation)?;

        Ok(token_data.claims)
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

    pub async fn get_sessions(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Session>> {
        self.sdk.get_models((), query_args).await
    }

    pub async fn get_session(&self, session_pk_id: &str) -> SdkResult<Session> {
        self.sdk
            .request_data(
                Method::GET,
                self.sdk.get_url_path("get-session", true, [("sessionPkId", session_pk_id)])?,
                NO_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn is_session_duplicated(&self, session_pk_id: &str, session_id: &str) -> SdkResult<bool> {
        self.sdk
            .request_data(
                Method::GET,
                self.sdk
                    .get_url_path("is-session-duplicated", true, [("sessionPkId", session_pk_id), ("sessionId", session_id)])?,
                NO_BODY,
            )
            .await?
            .into_data_default()
    }
}

#[cfg(test)]

mod tests {
    use std::fs;

    use crate::Config;

    #[test]
    fn succesfully_es256() {
        let token = fs::read_to_string("./src/authn/testdata/tok_es256.txt").unwrap();
        let cert = fs::read_to_string("./src/authn/testdata/cert_es256.txt").unwrap();
        let cfg = Config::new(
            "http://localhost:8000", 
            "e953686f04e7055b698b", 
            "secret", 
            cert, 
            "org_name", 
            Some("app_name".to_owned()),
        ).into_sdk();

        let authnx = cfg.authn();

        let tk = authnx.parse_jwt_token_es256(&token).unwrap();
        assert_eq!("user1", tk.user.display_name);
    }

    #[test]
    fn succesfully_es384() {
        let token = fs::read_to_string("./src/authn/testdata/tok_es384.txt").unwrap();
        let cert = fs::read_to_string("./src/authn/testdata/cert_es384.txt").unwrap();
        let cfg = Config::new(
            "http://localhost:8000", 
            "e953686f04e7055b698b", 
            "secret", 
            cert, 
            "org_name", 
            Some("app_name".to_owned()),
        ).into_sdk();

        let authnx = cfg.authn();

        let tk = authnx.parse_jwt_token_es384(&token).unwrap();
        assert_eq!("user1", tk.user.display_name);
    }

    #[test]
    fn succesfully_rs512() {
        let token = fs::read_to_string("./src/authn/testdata/tok_rs512.txt").unwrap();
        let cert = fs::read_to_string("./src/authn/testdata/cert_rs512.txt").unwrap();
        let cfg = Config::new(
            "http://localhost:8000", 
            "e953686f04e7055b698b", 
            "secret", 
            cert, 
            "org_name", 
            Some("app_name".to_owned()),
        ).into_sdk();

        let authnx = cfg.authn();

        let tk = authnx.parse_jwt_token_rs512(&token).unwrap();
        assert_eq!("user1", tk.user.display_name);
    }

    #[test]
    #[should_panic]
    fn bad_algo_rs_tk256_cert512() {
        let token = fs::read_to_string("./src/authn/testdata/tok_rs256.txt").unwrap();
        let cert = fs::read_to_string("./src/authn/testdata/cert_rs512.txt").unwrap();
        let cfg = Config::new(
            "http://localhost:8000", 
            "e953686f04e7055b698b", 
            "secret", 
            cert, 
            "org_name", 
            Some("app_name".to_owned()),
        ).into_sdk();

        let authnx = cfg.authn();

        let _tk = authnx.parse_jwt_token_rs512(&token).unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_algo_es_tk256_cert512() {
        let token = fs::read_to_string("./src/authn/testdata/tok_es256.txt").unwrap();
        let cert = fs::read_to_string("./src/authn/testdata/cert_es384.txt").unwrap();
        let cfg = Config::new(
            "http://localhost:8000", 
            "e953686f04e7055b698b", 
            "secret", 
            cert, 
            "org_name", 
            Some("app_name".to_owned()),
        ).into_sdk();

        let authnx = cfg.authn();

        let _tk = authnx.parse_jwt_token_es384(&token).unwrap();
    }
}