// Copyright 2022 The Casdoor Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use super::null_to_default;
use serde::{Deserialize, Serialize};

/// User info struct, defined in the SDK.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct CasdoorUser {
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
