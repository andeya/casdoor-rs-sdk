use serde::{Deserialize, Serialize};

use crate::Model;

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Cert {
    owner: String,
    name: String,
    created_time: String,

    display_name: String,
    scope: String,
    type_: String,
    crypto_algorithm: String,
    bit_size: i32,
    expire_in_years: i32,

    certificate: String,
    private_key: String,
    authority_public_key: String,
    authority_root_public_key: String,
}

impl Model for Cert {
    fn ident() -> &'static str {
        "cert"
    }
    fn plural_ident() -> &'static str {
        "certs"
    }
    fn support_update_columns() -> bool {
        false
    }
    fn owner(&self) -> &str {
        &self.owner
    }
    fn name(&self) -> &str {
        &self.name
    }
}
