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

use serde::{Deserialize, Serialize};

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

#[allow(dead_code)]
impl<'a> UserService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<CasdoorUser>> {
        let url = format!(
            "{}/api/get-users?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let res: ApiResponse<Vec<CasdoorUser>> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data_default()
    }

    pub async fn get_sorted_users(
        &self,
        sorter: String,
        limit: i32,
    ) -> anyhow::Result<Vec<CasdoorUser>> {
        let url = format!(
            "{}/api/get-sorted-users?owner={}&clientId={}&clientSecret={}&sorter={}&limit={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            sorter,
            limit
        );

        let res: ApiResponse<Vec<CasdoorUser>> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data_default()
    }

    pub async fn get_user_count(&self, is_online: QueryUserSet) -> anyhow::Result<i64> {
        let url = format!(
            "{}/api/get-user-count?owner={}&clientId={}&clientSecret={}&isOnline={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            is_online
        );
        let res: ApiResponse<i64> = reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data_default()
    }

    pub async fn get_user(&self, name: String) -> anyhow::Result<Option<CasdoorUser>> {
        let url = format!(
            "{}/api/get-user?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let res: ApiResponse<CasdoorUser> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data()
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<CasdoorUser>> {
        let url = format!(
            "{}/api/get-user?owner={}&clientId={}&clientSecret={}&email={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            email
        );

        let res: ApiResponse<CasdoorUser> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> anyhow::Result<Option<CasdoorUser>> {
        let url = format!(
            "{}/api/get-user?owner={}&clientId={}&clientSecret={}&phone={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            phone
        );

        let res: ApiResponse<CasdoorUser> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data()
    }

    pub async fn get_user_by_id(&self, user_id: String) -> anyhow::Result<Option<CasdoorUser>> {
        let url = format!(
            "{}/api/get-user?owner={}&clientId={}&clientSecret={}&userId={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            user_id
        );

        let res: ApiResponse<CasdoorUser> =
            reqwest::Client::new().get(url).send().await?.json().await?;
        res.into_data()
    }

    pub async fn modify_user(&self, args: ModifyUserArgs) -> anyhow::Result<UserOpAction> {
        let mut url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            args.action,
            args.user.owner,
            args.user.name,
            self.config.client_id,
            self.config.client_secret
        );
        if args.columns.len() > 0 {
            url += &format!("&columns={}", args.columns.join(","));
        }

        let res: ApiResponse<UserOpAction> = reqwest::Client::new()
            .post(url)
            .json(&args.user)
            .send()
            .await?
            .json()
            .await?;
        res.into_data_default()
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
