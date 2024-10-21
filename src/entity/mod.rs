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

mod config;
mod user;

pub use crate::entity::config::*;
pub use crate::entity::user::*;
use serde::de::Deserializer;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

pub fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiResponse<Data, Data2 = ()> {
    pub data: Option<Data>,
    pub data2: Option<Data2>,
    pub name: String,
    #[serde(flatten)]
    pub status: Status,
    pub sub: String,
}

#[derive(Debug)]
pub enum Status {
    Ok(String),
    Err(String),
    Other { status: String, msg: String },
}

impl Default for Status {
    fn default() -> Self {
        Self::Ok(Default::default())
    }
}
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StatusContainer {
            status: String,
            msg: String,
        }

        let container = StatusContainer::deserialize(deserializer)?;
        match container.status.as_str() {
            "ok" => Ok(Status::Ok(container.msg)),
            "error" => Ok(Status::Err(container.msg)),
            _ => Ok(Status::Other {
                status: container.status,
                msg: container.msg,
            }),
        }
    }
}
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create a new struct to serialize the enum variants
        let mut state = serializer.serialize_struct("Status", 2)?;
        match self {
            Status::Ok(msg) => {
                state.serialize_field("status", "ok")?;
                state.serialize_field("msg", msg)?;
            }
            Status::Err(msg) => {
                state.serialize_field("status", "error")?;
                state.serialize_field("msg", msg)?;
            }
            Status::Other { status, msg } => {
                state.serialize_field("status", status)?;
                state.serialize_field("msg", msg)?;
            }
        }
        state.end()
    }
}

impl<Data, Data2> ApiResponse<Data, Data2> {
    pub fn into_result(self) -> anyhow::Result<(Option<Data>, Option<Data2>)> {
        match self.status {
            Status::Ok(_) => Ok((self.data, self.data2)),
            Status::Err(e) => Err(anyhow::anyhow!(e)),
            Status::Other { status, msg } => {
                Err(anyhow::anyhow!("Unknown: status={status}, msg={msg}"))
            }
        }
    }

    pub fn into_result_default(self) -> anyhow::Result<(Data, Data2)>
    where
        Data: Default,
        Data2: Default,
    {
        let (data, data2) = self.into_result()?;
        Ok((data.unwrap_or_default(), data2.unwrap_or_default()))
    }

    pub fn into_data(self) -> anyhow::Result<Option<Data>> {
        let (data, _) = self.into_result()?;
        Ok(data)
    }

    pub fn into_data_value(self) -> anyhow::Result<Data> {
        let (data, _) = self.into_result()?;
        data.ok_or(anyhow::anyhow!("Unexpected empty data."))
    }

    pub fn into_data_default(self) -> anyhow::Result<Data>
    where
        Data: Default,
    {
        let (data, _) = self.into_result()?;
        Ok(data.unwrap_or_default())
    }

    pub fn into_data2(self) -> anyhow::Result<Option<Data2>> {
        let (_, data2) = self.into_result()?;
        Ok(data2)
    }

    pub fn into_data2_value(self) -> anyhow::Result<Data2> {
        let (_, data2) = self.into_result()?;
        data2.ok_or(anyhow::anyhow!("Unexpected empty data2."))
    }

    pub fn into_data2_default(self) -> anyhow::Result<Data2>
    where
        Data2: Default,
    {
        let (_, data2) = self.into_result()?;
        Ok(data2.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_res_json() {
        let json_data = r#"{"data":{"accessKey":"test"},"data2":null,"name":"","status":"ok","msg":"test","sub":""}"#;
        let obj: ApiResponse<HashMap<String, String>, ()> =
            serde_json::from_str(&json_data).unwrap();
        println!("{obj:?}");
        let json_data2 = serde_json::to_string(&obj).unwrap();
        assert_eq!(json_data, json_data2);
    }

    #[test]
    fn test_user_json() {
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
