use std::fmt::Display;

use crate::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SdkError {
    pub code: StatusCode,
    pub message: String,
}

impl SdkError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SdkError {}

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
            message: e.to_string(),
        }
    }
}

impl From<serde_urlencoded::ser::Error> for SdkError {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, value.to_string())
    }
}

impl From<oauth2::url::ParseError> for SdkError {
    fn from(value: oauth2::url::ParseError) -> Self {
        Self::new(StatusCode::BAD_REQUEST, value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for SdkError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, value.to_string())
    }
}

impl<RE, TE> From<oauth2::RequestTokenError<RE, TE>> for SdkError
where
    RE: std::error::Error + 'static,
    TE: oauth2::ErrorResponse + 'static,
{
    fn from(value: oauth2::RequestTokenError<RE, TE>) -> Self {
        match value {
            oauth2::RequestTokenError::ServerResponse(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string()),
            oauth2::RequestTokenError::Request(_) => Self::new(StatusCode::BAD_REQUEST, value.to_string()),
            oauth2::RequestTokenError::Parse(_, _) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string()),
            oauth2::RequestTokenError::Other(_) => Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string()),
        }
    }
}

#[cfg(feature = "salvo")]
impl SdkError {
    fn into_salvo(self) -> salvo::prelude::StatusError {
        use salvo::prelude::StatusError;
        StatusError::from_code(self.code)
            .unwrap_or(StatusError::internal_server_error())
            .brief(self.message)
    }
}

#[cfg(feature = "salvo")]
impl From<SdkError> for salvo::prelude::StatusError {
    fn from(value: SdkError) -> Self {
        value.into_salvo()
    }
}
