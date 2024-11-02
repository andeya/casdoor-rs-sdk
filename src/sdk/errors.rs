use std::fmt::Display;

use cubix::api_response::error_code::CodeSegment::{S98, S99};
pub use cubix::api_response::{
    error_code::{CodeSegment, ErrorCode},
    ApiError,
};

use crate::StatusCode;

#[derive(Debug)]
#[non_exhaustive]
pub struct SdkError {
    pub code: StatusCode,
    pub inner: SdkInnerError,
}

impl SdkError {
    pub fn new(code: StatusCode, inner: impl Into<SdkInnerError>) -> Self {
        Self { code, inner: inner.into() }
    }
}

impl Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for SdkError {}

#[derive(Debug)]
pub enum SdkInnerError {
    StringError(String),
    ReqwestError(reqwest::Error),
    SerdeUrlencodedSerError(serde_urlencoded::ser::Error),
    Oauth2UrlParseError(oauth2::url::ParseError),
    Oauth2RequestTokenError(String),
    JwtError(jsonwebtoken::errors::Error),
}

impl Display for SdkInnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdkInnerError::StringError(error) => write!(f, "{error}"),
            SdkInnerError::ReqwestError(error) => write!(f, "{error}"),
            SdkInnerError::SerdeUrlencodedSerError(error) => write!(f, "{error}"),
            SdkInnerError::Oauth2UrlParseError(error) => write!(f, "{error}"),
            SdkInnerError::Oauth2RequestTokenError(error) => write!(f, "{error}"),
            SdkInnerError::JwtError(error) => write!(f, "{error}"),
        }
    }
}

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
            inner: SdkInnerError::ReqwestError(e),
        }
    }
}

impl<T: Into<String>> From<T> for SdkInnerError {
    fn from(value: T) -> Self {
        Self::StringError(value.into())
    }
}

impl From<serde_urlencoded::ser::Error> for SdkError {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkInnerError::SerdeUrlencodedSerError(value))
    }
}

impl From<oauth2::url::ParseError> for SdkError {
    fn from(value: oauth2::url::ParseError) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkInnerError::Oauth2UrlParseError(value))
    }
}

impl From<jsonwebtoken::errors::Error> for SdkError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(StatusCode::BAD_REQUEST, SdkInnerError::JwtError(value))
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
            .brief(self.inner.to_string())
    }
}

#[cfg(feature = "salvo")]
impl From<SdkError> for salvo::prelude::StatusError {
    fn from(value: SdkError) -> Self {
        value.into_salvo()
    }
}

static mut ERR_CODE_SUFFIX: (CodeSegment, CodeSegment, CodeSegment) = (S99, S99, S98);

/// Initialize the suffix of the error code.
/// # Safety
///  It must be called in advance in a synchronous environment.
/// # Example:
/// ```
/// const _: () = unsafe { init_error_code_suffix(S99, S99, S98) };
/// ```
pub const unsafe fn init_error_code_suffix(s1: CodeSegment, s2: CodeSegment, s3: CodeSegment) {
    ERR_CODE_SUFFIX = (s1, s2, s3)
}

impl From<SdkError> for ApiError {
    fn from(value: SdkError) -> Self {
        let (s1, s2, s3) = unsafe { ERR_CODE_SUFFIX };
        ErrorCode::from(value.code).api_error3(s1, s2, s3, value.to_string())
    }
}
