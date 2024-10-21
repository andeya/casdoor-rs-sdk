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

use std::fmt::Display;

use http::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::entity::{ApiResponse, CasdoorConfig, CasdoorUser};

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
    action: UserAction,
    user: CasdoorUser,
    columns: Vec<String>,
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

pub struct UserService<'a> {
    config: &'a CasdoorConfig,
}
const NONE_BODY: Option<&()> = None::<&()>;
#[allow(dead_code)]
impl<'a> UserService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    async fn new_request<Data: DeserializeOwned + Default>(
        &self,
        method: Method,
        url_path: impl AsRef<str>,
        body: Option<&impl Serialize>,
    ) -> reqwest::Result<ApiResponse<Data, ()>> {
        let mut req = reqwest::Client::new()
            .request(method, self.config.endpoint.clone() + url_path.as_ref())
            .basic_auth(
                self.config.client_id.clone(),
                Some(self.config.client_secret.clone()),
            );
        if let Some(body) = body {
            req = req.json(body);
        }
        req.send().await?.json::<ApiResponse<Data, ()>>().await
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!("/api/get-users?owner={}", self.config.org_name),
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }

    pub async fn get_sorted_users(
        &self,
        sorter: String,
        limit: i32,
    ) -> anyhow::Result<Vec<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!(
                "/api/get-sorted-users?owner={}&sorter={}&limit={}",
                self.config.org_name, sorter, limit
            ),
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }

    pub async fn get_user_count(&self, is_online: QueryUserSet) -> anyhow::Result<i64> {
        self.new_request(
            Method::GET,
            format!(
                "/api/get-user-count?owner={}&isOnline={}",
                self.config.org_name, is_online
            ),
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }

    pub async fn get_user(&self, name: String) -> anyhow::Result<Option<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!("/api/get-user?id={}/{}", self.config.org_name, name),
            NONE_BODY,
        )
        .await?
        .into_data()
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!(
                "/api/get-user?owner={}&email={}",
                self.config.org_name, email
            ),
            NONE_BODY,
        )
        .await?
        .into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> anyhow::Result<Option<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!(
                "/api/get-user?owner={}&phone={}",
                self.config.org_name, phone
            ),
            NONE_BODY,
        )
        .await?
        .into_data()
    }

    pub async fn get_user_by_user_id(
        &self,
        user_id: String,
    ) -> anyhow::Result<Option<CasdoorUser>> {
        self.new_request(
            Method::GET,
            format!(
                "/api/get-user?owner={}&userId={}",
                self.config.org_name, user_id
            ),
            NONE_BODY,
        )
        .await?
        .into_data()
    }

    pub async fn modify_user(&self, args: ModifyUserArgs) -> anyhow::Result<UserOpAction> {
        let mut url_path = format!(
            "/api/{}?id={}/{}",
            args.action, args.user.owner, args.user.name,
        );
        if args.columns.len() > 0 {
            url_path += &format!("&columns={}", args.columns.join(","));
        }
        self.new_request(Method::POST, url_path, Some(&args.user))
            .await?
            .into_data_default()
    }

    pub async fn add_user(
        &self,
        user: CasdoorUser,
        columns: Vec<String>,
    ) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Add,
            user,
            columns,
        })
        .await
    }

    pub async fn delete_user(
        &self,
        user: CasdoorUser,
        columns: Vec<String>,
    ) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Delete,
            user,
            columns,
        })
        .await
    }

    pub async fn update_user(
        &self,
        user: CasdoorUser,
        columns: Vec<String>,
    ) -> anyhow::Result<UserOpAction> {
        self.modify_user(ModifyUserArgs {
            action: UserAction::Update,
            user,
            columns,
        })
        .await
    }
}
