use jwt_claims::RegisteredClaims;
pub use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use crate::User;

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Claims {
    #[serde(flatten)]
    pub user: User,
    pub access_token: String,
    pub tag: String,
    pub token_type: Option<String>,
    pub nonce: Option<String>,
    pub scope: Option<String>,
    #[serde(flatten)]
    pub reg_claims: RegisteredClaims,
}
