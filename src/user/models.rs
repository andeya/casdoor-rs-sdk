use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{IsQueryArgs, Model, Permission, Role, utils::null_to_default};

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub deleted_time: String,
    pub id: String,
    pub external_id: String,
    pub r#type: String,
    pub password: String,
    pub password_salt: String,
    pub password_type: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar: String,
    pub avatar_type: String,
    pub permanent_avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub phone: String,
    pub country_code: String,
    pub region: String,
    pub location: String,
    #[serde(deserialize_with = "null_to_default")]
    pub address: Vec<String>,
    pub affiliation: String,
    pub title: String,
    pub id_card_type: String,
    pub id_card: String,
    pub homepage: String,
    pub bio: String,
    pub tag: String,
    pub language: String,
    pub gender: String,
    pub birthday: String,
    pub education: String,
    pub score: i32,
    pub karma: i32,
    pub ranking: i32,
    pub balance: f64,
    pub currency: String,
    pub is_default_avatar: bool,
    pub is_online: bool,
    pub is_admin: bool,
    pub is_forbidden: bool,
    pub is_deleted: bool,
    pub signup_application: String,
    pub hash: String,
    pub pre_hash: String,
    pub access_key: String,
    pub access_secret: String,
    pub access_token: String,
    pub created_ip: String,
    pub last_signin_time: String,
    pub last_signin_ip: String,
    pub github: String,
    pub google: String,
    pub qq: String,
    pub wechat: String,
    pub facebook: String,
    pub dingtalk: String,
    pub weibo: String,
    pub gitee: String,
    pub linkedin: String,
    pub wecom: String,
    pub lark: String,
    pub gitlab: String,
    pub adfs: String,
    pub baidu: String,
    pub alipay: String,
    pub casdoor: String,
    pub infoflow: String,
    pub apple: String,
    pub azuread: String,
    pub azureadb2c: String,
    pub slack: String,
    pub steam: String,
    pub bilibili: String,
    pub okta: String,
    pub douyin: String,
    pub line: String,
    pub amazon: String,
    pub auth0: String,
    pub battlenet: String,
    pub bitbucket: String,
    pub r#box: String,
    pub cloudfoundry: String,
    pub dailymotion: String,
    pub deezer: String,
    pub digitalocean: String,
    pub discord: String,
    pub dropbox: String,
    pub eveonline: String,
    pub fitbit: String,
    pub gitea: String,
    pub heroku: String,
    pub influxcloud: String,
    pub instagram: String,
    pub intercom: String,
    pub kakao: String,
    pub lastfm: String,
    pub mailru: String,
    pub meetup: String,
    pub microsoftonline: String,
    pub naver: String,
    pub nextcloud: String,
    pub onedrive: String,
    pub oura: String,
    pub patreon: String,
    pub paypal: String,
    pub salesforce: String,
    pub shopify: String,
    pub soundcloud: String,
    pub spotify: String,
    pub strava: String,
    pub stripe: String,
    pub tiktok: String,
    pub tumblr: String,
    pub twitch: String,
    pub twitter: String,
    pub typetalk: String,
    pub uber: String,
    pub vk: String,
    pub wepay: String,
    pub xero: String,
    pub yahoo: String,
    pub yammer: String,
    pub yandex: String,
    pub zoom: String,
    pub metamask: String,
    pub web3onboard: String,
    pub custom: String,
    #[serde(deserialize_with = "null_to_default")]
    pub webauthn_credentials: Vec<String>,
    pub preferred_mfa_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub totp_secret: String,
    pub mfa_phone_enabled: bool,
    pub mfa_email_enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub multi_factor_auths: Vec<MfaProps>,
    pub invitation: String,
    pub invitation_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub face_ids: Vec<FaceId>,
    pub ldap: String,
    pub properties: HashMap<String, String>,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<Role>,
    #[serde(deserialize_with = "null_to_default")]
    pub permissions: Vec<Permission>,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    pub last_signin_wrong_time: String,
    pub signin_wrong_times: i32,
    #[serde(deserialize_with = "null_to_default")]
    pub managed_accounts: Vec<ManagedAccount>,
    #[serde(deserialize_with = "null_to_default")]
    pub mfa_accounts: Vec<MfaAccount>,
    pub need_update_password: bool,
    pub ip_whitelist: String,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FaceId {
    pub name: String,
    pub face_id_data: Vec<f64>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaProps {
    pub enabled: bool,
    pub is_preferred: bool,
    pub mfa_type: String,
    pub secret: Option<String>,
    pub country_code: Option<String>,
    pub url: Option<String>,
    pub recovery_codes: Option<Vec<String>>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaAccount {
    pub account_name: String,
    pub issuer: String,
    pub secret_key: String,
}

impl Model for User {
    fn ident() -> &'static str {
        "user"
    }
    fn plural_ident() -> &'static str {
        "users"
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
pub struct UserGroup {
    owner: String,
    name: String,
    created_time: String,
    updated_time: String,

    display_name: String,
    manager: String,
    contact_email: String,
    type_: String,
    parent_id: String,
    is_top_group: bool,
    users: Vec<String>,

    title: Option<String>,
    key: Option<String>,
    children: Option<Vec<UserGroup>>,

    is_enabled: bool,
}

impl Model for UserGroup {
    fn ident() -> &'static str {
        "group"
    }
    fn plural_ident() -> &'static str {
        "groups"
    }
    fn owner(&self) -> &str {
        &self.owner
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn support_update_columns() -> bool {
        false
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

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserQueryArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// page
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
}
impl IsQueryArgs for UserQueryArgs {}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetUserArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserGroupQueryArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "withTree", skip_serializing_if = "Option::is_none")]
    pub with_tree: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// page
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
}
impl IsQueryArgs for UserGroupQueryArgs {}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetPasswordArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    pub user_name: String,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    pub new_password: String,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query,required=false)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_owner: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_query_args() {
        let mut args = UserQueryArgs::default();
        let query_part = serde_urlencoded::to_string(&args).unwrap();
        assert_eq!("", query_part);

        args.page = Some(0);
        args.page_size = Some(1);
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
