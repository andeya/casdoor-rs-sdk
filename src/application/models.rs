use serde::{Deserialize, Serialize};

use crate::{Cert, IsQueryArgs, Model, Organization, Provider, ThemeData};

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Application {
    owner: String,
    name: String,
    created_time: String,

    display_name: String,
    logo: String,
    homepage_url: String,
    description: String,
    organization: String,
    cert: String,
    header_html: String,
    enable_password: bool,
    enable_sign_up: bool,
    enable_signin_session: bool,
    enable_auto_signin: bool,
    enable_code_signin: bool,
    enable_saml_compress: bool,
    enable_saml_c14n10: bool,
    enable_saml_post_binding: bool,
    use_email_as_saml_name_id: bool,
    enable_web_authn: bool,
    enable_link_with_email: bool,
    org_choice_mode: String,
    saml_reply_url: String,
    providers: Vec<ProviderItem>,
    signin_methods: Vec<SigninMethod>,
    signup_items: Vec<SignupItem>,
    signin_items: Vec<SigninItem>,
    grant_types: Vec<String>,
    organization_obj: Option<Organization>,
    cert_public_key: String,
    tags: Vec<String>,
    saml_attributes: Vec<SamlItem>,
    is_shared: bool,

    client_id: String,
    client_secret: String,
    redirect_uris: Vec<String>,
    token_format: String,
    token_signing_method: String,
    token_fields: Vec<String>,
    expire_in_hours: i32,
    refresh_expire_in_hours: i32,
    signup_url: String,
    signin_url: String,
    forget_url: String,
    affiliation_url: String,
    terms_of_use: String,
    signup_html: String,
    signin_html: String,
    theme_data: Option<ThemeData>,
    footer_html: String,
    form_css: String,
    form_css_mobile: String,
    form_offset: i32,
    form_side_html: String,
    form_background_url: String,

    failed_signin_limit: i32,
    failed_signin_frozen_time: i32,

    cert_obj: Option<Cert>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct ProviderItem {
    owner: String,
    name: String,

    can_sign_up: bool,
    can_sign_in: bool,
    can_unlink: bool,
    prompted: bool,
    alert_type: String,
    rule: String,
    provider: Option<Provider>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct SignupItem {
    name: String,
    visible: bool,
    required: bool,
    prompted: bool,
    custom_css: String,
    label: String,
    placeholder: String,
    regex: String,
    rule: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct SigninMethod {
    name: String,
    display_name: String,
    rule: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct SigninItem {
    name: String,
    visible: bool,
    label: String,
    custom_css: String,
    placeholder: String,
    rule: String,
    is_custom: bool,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct SamlItem {
    name: String,
    name_format: String,
    value: String,
}

impl Model for Application {
    fn ident() -> &'static str {
        "application"
    }
    fn plural_ident() -> &'static str {
        "applications"
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

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ApplicationQueryArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "sortField", skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    pub organization_name: Option<String>,
}

impl IsQueryArgs for ApplicationQueryArgs {}
