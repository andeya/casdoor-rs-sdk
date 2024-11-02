use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Model;

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Provider {
    owner: String,
    name: String,
    created_time: String,

    display_name: String,
    category: String,
    type_: String,
    sub_type: String,
    method: String,
    client_id: String,
    client_secret: String,
    client_id2: String,
    client_secret2: String,
    cert: String,
    custom_auth_url: String,
    custom_token_url: String,
    custom_user_info_url: String,
    custom_logo: String,
    scopes: String,
    user_mapping: HashMap<String, String>,

    host: String,
    port: i32,
    disable_ssl: bool, // If the provider type is WeChat, DisableSsl means EnableQRCode
    title: String,
    content: String, // If provider type is WeChat, Content means QRCode string by Base64 encoding
    receiver: String,

    region_id: String,
    sign_name: String,
    template_code: String,
    app_id: String,

    endpoint: String,
    intranet_endpoint: String,
    domain: String,
    bucket: String,
    path_prefix: String,

    metadata: String,
    idp: String,
    issuer_url: String,
    enable_sign_authn_request: bool,

    provider_url: String,
}

impl Model for Provider {
    fn ident() -> &'static str {
        "provider"
    }
    fn plural_ident() -> &'static str {
        "providers"
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
