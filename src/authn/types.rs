pub use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use crate::User;

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Claims {
    #[serde(flatten)]
    pub user: User,
    pub access_token: String,
    pub token_type: String,
    pub refresh_token_type: String,
}
