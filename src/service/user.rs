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

use crate::entity::{ApiResponse, CasdoorConfig, CasdoorUser, UserOpAction};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UserOp {
    Add,
    Delete,
    Update,
}

impl Display for UserOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserOp::Add => write!(f, "add-user"),
            UserOp::Delete => write!(f, "delete-user"),
            UserOp::Update => write!(f, "update-user"),
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

    pub async fn get_user_count(&self, is_online: String) -> anyhow::Result<i64> {
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

    pub async fn get_user_with_email(&self, email: String) -> anyhow::Result<Option<CasdoorUser>> {
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

    pub async fn modify_user(
        &self,
        op: UserOp,
        user: &CasdoorUser,
    ) -> anyhow::Result<UserOpAction> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            user.owner,
            user.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res: ApiResponse<UserOpAction> = reqwest::Client::new()
            .post(url)
            .json(user)
            .send()
            .await?
            .json()
            .await?;
        res.into_data_default()
    }

    pub async fn add_user(&self, user: &CasdoorUser) -> anyhow::Result<UserOpAction> {
        self.modify_user(UserOp::Add, user).await
    }

    pub async fn delete_user(&self, user: &CasdoorUser) -> anyhow::Result<UserOpAction> {
        self.modify_user(UserOp::Delete, user).await
    }

    pub async fn update_user(&self, user: &CasdoorUser) -> anyhow::Result<UserOpAction> {
        self.modify_user(UserOp::Update, user).await
    }
}
