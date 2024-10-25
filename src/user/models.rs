use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{utils::null_to_default, Model, QueryArgs};

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

impl Model for User {
    fn ident() -> &'static str {
        "user"
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserQueryArgs {
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(flatten)]
    pub base: QueryArgs,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_query_args() {
        let mut args = UserQueryArgs::default();
        let query_part = serde_urlencoded::to_string(&args).unwrap();
        assert_eq!("", query_part);

        args.base.page = Some(0);
        args.base.page_size = Some(1);
        let query_part = serde_urlencoded::to_string(&args).unwrap();
        assert_eq!("pageSize=1&p=0", query_part);
    }
    #[test]
    fn test_user() {
        let json_data = r#"
    {
        "owner": "example_owner",
        "name": "example_name",
        "createdTime": "2022-01-01T00:00:00Z",
        "updatedTime": "2022-01-01T00:00:00Z",
        
        "id": "example_id",
        "type": "example_type",
        "password": "example_password",
        "passwordSalt": "example_salt",
        "passwordType": "example_type",
        "displayName": "Example User",
        "firstName": "First",
        "lastName": "Last",
        "avatar": "example_avatar",
        "avatarType": "example_type",
        "permanentAvatar": "example_perm_avatar",
        "email": "example@example.com",
        "emailVerified": true,
        "phone": "123456789",
        "countryCode": "example_cc",
        "region": "example_region",
        "location": "Example Location",
        "address": ["Example Address"],
        "affiliation": "example_affiliation",
        "title": "example_title",
        "idCardType": "example_card_type",
        "idCard": "example_card",
        "homepage": "https://example.com",
        "bio": "Example bio",
        "tag": "example_tag",
        "language": "en",
        "gender": "M",
        "birthday": "1990-01-01",
        "education": "example_education",
        "score": 100,
        "karma": 10,
        "ranking": 1,
        "isDefaultAvatar": true,
        "isOnline": true,
        "isAdmin": true,
        "isForbidden": false,
        "isDeleted": false,
        "signupApplication": "example_signup_app",
        "hash": "example_hash",
        "preHash": "example_pre_hash",
        
        "github": "example_github",
        "google": "example_google",
        "qq": "example_qq",
        "wechat": "example_wechat",
        "facebook": "example_facebook",
        "dingtalk": "example_dingtalk",
        "weibo": "example_weibo",
        "gitee": "example_gitee",
        "linkedin": "example_linkedin",
        "wecom": "example_wecom",
        "lark": "example_lark",
        "gitlab": "example_gitlab",
        "ldap": "example_ldap",

        "properties": {
            "additionalProp1": "value1",
            "additionalProp2": "value2",
            "additionalProp3": "value3"
        },
        "groups": ["ExampleGroup"],
        "lastSigninWrongTime": "2022-01-01T00:00:00Z",
        "signinWrongTimes": 0
    }
    "#;

        let casdoor_user: User = serde_json::from_str(json_data).expect("JSON parsing failed");
        println!("{:?}", casdoor_user);
    }
}
