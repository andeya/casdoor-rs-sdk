use jwt_claims::RegisteredClaims;
pub use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use crate::{Model, User};

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

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Session {
    owner: String,
    name: String,
    application: String,
    created_time: String,
    session_id: Vec<String>,
}

impl Session {
    pub fn get_pk_id(&self) -> String {
        format!("{}/{}/{}", self.owner, self.name, self.application)
    }
}

impl Model for Session {
    fn ident() -> &'static str {
        "session"
    }

    fn owner(&self) -> &str {
        &self.owner
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn support_update_columns() -> bool {
        true
    }
}
