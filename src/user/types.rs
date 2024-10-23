use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::utils::null_to_default;

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Userinfo {
    sub: String,
    iss: String,
    aud: String,
    #[serde(rename = "preferred_username", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(rename = "email_verified", skip_serializing_if = "Option::is_none")]
    email_verified: Option<bool>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
}

/// User info struct, defined in the SDK.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub access_key: String,
    pub access_secret: String,
    #[serde(deserialize_with = "null_to_default")]
    pub address: Vec<String>,
    pub adfs: String,
    pub affiliation: String,
    pub alipay: String,
    pub amazon: String,
    pub apple: String,
    pub auth0: String,
    pub avatar: String,
    pub avatar_type: String,
    pub azuread: String,
    pub azureadb2c: String,
    pub baidu: String,
    pub battlenet: String,
    pub bilibili: String,
    pub bio: String,
    pub birthday: String,
    pub bitbucket: String,
    pub r#box: String,
    pub casdoor: String,
    pub cloudfoundry: String,
    pub country_code: String,
    pub created_ip: String,
    pub created_time: String,
    pub custom: String,
    pub dailymotion: String,
    pub deezer: String,
    pub digitalocean: String,
    pub dingtalk: String,
    pub discord: String,
    pub display_name: String,
    pub deleted_time: String,
    pub douyin: String,
    pub dropbox: String,
    pub education: String,
    pub email: String,
    pub email_verified: bool,
    pub eveonline: String,
    pub external_id: String,
    pub facebook: String,
    pub first_name: String,
    pub fitbit: String,
    pub gender: String,
    pub gitea: String,
    pub gitee: String,
    pub github: String,
    pub gitlab: String,
    pub google: String,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    pub hash: String,
    pub heroku: String,
    pub homepage: String,
    pub id: String,
    pub id_card: String,
    pub id_card_type: String,
    pub influxcloud: String,
    pub infoflow: String,
    pub instagram: String,
    pub intercom: String,
    pub is_admin: bool,
    pub is_default_avatar: bool,
    pub is_deleted: bool,
    pub is_forbidden: bool,
    pub is_online: bool,
    pub kakao: String,
    pub karma: i32,
    pub language: String,
    pub lark: String,
    pub last_name: String,
    pub last_signin_ip: String,
    pub last_signin_time: String,
    pub last_signin_wrong_time: String,
    pub lastfm: String,
    pub ldap: String,
    pub line: String,
    pub linkedin: String,
    pub location: String,
    pub mailru: String,
    #[serde(deserialize_with = "null_to_default")]
    pub managed_accounts: Vec<ManagedAccount>,
    pub meetup: String,
    pub metamask: String,
    pub mfa_email_enabled: bool,
    pub mfa_phone_enabled: bool,
    pub microsoftonline: String,
    #[serde(deserialize_with = "null_to_default")]
    pub multi_factor_auths: Vec<MultiFactorAuth>,
    pub name: String,
    pub naver: String,
    pub nextcloud: String,
    pub okta: String,
    pub onedrive: String,
    pub oura: String,
    pub owner: String,
    pub password: String,
    pub password_salt: String,
    pub password_type: String,
    pub patreon: String,
    pub paypal: String,
    pub permanent_avatar: String,
    #[serde(deserialize_with = "null_to_default")]
    pub permissions: Vec<Permission>,
    pub phone: String,
    pub pre_hash: String,
    pub preferred_mfa_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub properties: HashMap<String, String>,
    pub qq: String,
    pub ranking: i32,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub region: String,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<Role>,
    pub salesforce: String,
    pub score: i32,
    pub shopify: String,
    pub signin_wrong_times: i32,
    pub signup_application: String,
    pub slack: String,
    pub soundcloud: String,
    pub spotify: String,
    pub steam: String,
    pub strava: String,
    pub stripe: String,
    pub tag: String,
    pub tiktok: String,
    pub title: String,
    pub totp_secret: String,
    pub tumblr: String,
    pub twitch: String,
    pub twitter: String,
    pub r#type: String,
    pub typetalk: String,
    pub uber: String,
    pub updated_time: String,
    pub vk: String,
    pub web3onboard: String,
    #[serde(deserialize_with = "null_to_default")]
    pub webauthn_credentials: Vec<WebauthnCredential>,
    pub wechat: String,
    pub wecom: String,
    pub weibo: String,
    pub wepay: String,
    pub xero: String,
    pub yahoo: String,
    pub yammer: String,
    pub yandex: String,
    pub zoom: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct ManagedAccount {
    pub application: String,
    pub password: String,
    pub signin_url: String,
    pub username: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct MultiFactorAuth {
    pub country_code: String,
    pub enabled: bool,
    pub is_preferred: bool,
    pub mfa_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub secret: String,
    pub url: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Permission {
    #[serde(deserialize_with = "null_to_default")]
    pub actions: Vec<String>,
    pub adapter: String,
    pub approve_time: String,
    pub approver: String,
    pub created_time: String,
    pub description: String,
    pub display_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub domains: Vec<String>,
    pub effect: String,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    pub is_enabled: bool,
    pub model: String,
    pub name: String,
    pub owner: String,
    pub resource_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub resources: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<String>,
    pub state: String,
    pub submitter: String,
    #[serde(deserialize_with = "null_to_default")]
    pub users: Vec<String>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Role {
    pub created_time: String,
    pub description: String,
    pub display_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub domains: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    pub is_enabled: bool,
    pub name: String,
    pub owner: String,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub users: Vec<String>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct WebauthnCredential {}

/// The filter for query user.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum QueryUserSet {
    /// 0 for offline
    Offline,
    /// 1 for online
    Online,
    /// empty for all users
    #[default]
    #[serde(other)]
    All,
}
impl Display for QueryUserSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryUserSet::Offline => write!(f, "0"),
            QueryUserSet::Online => write!(f, "1"),
            QueryUserSet::All => write!(f, ""),
        }
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyUserArgs {
    pub action: UserAction,
    pub user: User,
    pub columns: Vec<String>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UserAction {
    Add,
    Delete,
    Update,
}

impl Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserAction::Add => write!(f, "add-user"),
            UserAction::Delete => write!(f, "delete-user"),
            UserAction::Update => write!(f, "update-user"),
        }
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserOpAction {
    #[default]
    Affected,
    Unaffected,
}

impl Display for UserOpAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Affected => write!(f, "Affected"),
            Self::Unaffected => write!(f, "Unaffected"),
        }
    }
}
