use std::fmt::Display;

use crate::StatusCode;

#[derive(Debug)]
#[non_exhaustive]
pub struct SdkError {
    pub code: StatusCode,
    pub inner: SdkSourceError,
}

impl SdkError {
    pub fn new(code: StatusCode, inner: impl Into<SdkSourceError>) -> Self {
        Self { code, inner: inner.into() }
    }
    pub fn from_str(code: StatusCode, source: impl Into<String>) -> Self {
        Self {
            code,
            inner: SdkSourceError::StringError(StringError(source.into())),
        }
    }
}

impl Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for SdkError {}

#[derive(Debug)]
pub enum SdkSourceError {
    StringError(StringError),
    ReqwestError(reqwest::Error),
    SerdeUrlencodedSerError(serde_urlencoded::ser::Error),
    Oauth2UrlParseError(oauth2::url::ParseError),
    Oauth2RequestTokenError(Oauth2RequestTokenStandardBasicError),
    JwtError(jsonwebtoken::errors::Error),
}

impl Display for SdkSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SdkSourceError::StringError(string_error) => string_error.0.to_owned(),
                SdkSourceError::ReqwestError(error) => error.to_string(),
                SdkSourceError::SerdeUrlencodedSerError(error) => error.to_string(),
                SdkSourceError::Oauth2UrlParseError(parse_error) => parse_error.to_string(),
                SdkSourceError::Oauth2RequestTokenError(request_token_error) => request_token_error.to_string(),
                SdkSourceError::JwtError(error) => error.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct StringError(pub String);
impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for StringError {}

impl From<reqwest::Error> for SdkError {
    fn from(e: reqwest::Error) -> Self {
        Self {
            code: if let Some(status) = e.status() {
                status
            } else if e.is_timeout() {
                StatusCode::REQUEST_TIMEOUT
            } else if e.is_request() {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            inner: Box::new(e),
        }
    }
}

impl From<serde_urlencoded::ser::Error> for SdkError {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkSourceError::SerdeUrlencodedSerError(value))
    }
}

impl From<oauth2::url::ParseError> for SdkError {
    fn from(value: oauth2::url::ParseError) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkSourceError::Oauth2UrlParseError(value))
    }
}

impl From<jsonwebtoken::errors::Error> for SdkError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkSourceError::JwtError(value))
    }
}

pub type Oauth2RequestTokenStandardBasicError =
    oauth2::RequestTokenError<oauth2::reqwest::Error<oauth2::reqwest::Error>, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>;

impl From<Oauth2RequestTokenStandardBasicError> for SdkError {
    fn from(value: Oauth2RequestTokenStandardBasicError) -> Self {
        match value {
            oauth2::RequestTokenError::ServerResponse(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, SdkSourceError::Oauth2RequestTokenError(value))
            }
            oauth2::RequestTokenError::Request(_) => Self::new(StatusCode::BAD_REQUEST, SdkSourceError::Oauth2RequestTokenError(value)),
            oauth2::RequestTokenError::Parse(_, _) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, SdkSourceError::Oauth2RequestTokenError(value)),
            oauth2::RequestTokenError::Other(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, SdkSourceError::Oauth2RequestTokenError(value)),
        }
    }
}

// impl<RE, TE> From<oauth2::RequestTokenError<RE, TE>> for SdkError
// where
//     RE: std::error::Error + 'static,
//     TE: oauth2::ErrorResponse + 'static,
// {
//     fn from(value: oauth2::RequestTokenError<RE, TE>) -> Self {
//         match value {
//             oauth2::RequestTokenError::ServerResponse(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value),
//             oauth2::RequestTokenError::Request(_) => Self::new(StatusCode::BAD_REQUEST, value),
//             oauth2::RequestTokenError::Parse(_, _) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value),
//             oauth2::RequestTokenError::Other(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value),
//         }
//     }
// }

#[cfg(feature = "salvo")]
impl SdkError {
    fn into_salvo(self) -> salvo::prelude::StatusError {
        use salvo::prelude::StatusError;
        StatusError::from_code(self.code)
            .unwrap_or(StatusError::internal_server_error())
            .brief(self.inner.to_string())
    }
}

#[cfg(feature = "salvo")]
impl From<SdkError> for salvo::prelude::StatusError {
    fn from(value: SdkError) -> Self {
        value.into_salvo()
    }
}
