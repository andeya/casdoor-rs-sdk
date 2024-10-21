// Copyright 2021 The Casdoor Authors. All Rights Reserved.
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

use casdoor_rust_sdk::{CasdoorConfig, CasdoorUser, QueryUserSet, UserService};

fn abs_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join("tests").join(path);
    Ok(absolute_path.to_str().unwrap().to_string())
}

#[tokio::test]
async fn test_get_users() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let users = user_service.get_users().await.unwrap();
    assert!(!users.is_empty());
}

#[tokio::test]
async fn test_get_sorted_users() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let users = user_service
        .get_sorted_users("name".to_string(), 1)
        .await
        .unwrap();
    assert!(!users.is_empty());
}

#[tokio::test]
async fn test_get_user_count() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let count = user_service
        .get_user_count(QueryUserSet::Offline)
        .await
        .unwrap();
    assert!(count == 1);
}

#[tokio::test]
async fn test_get_user() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user("admin".to_string())
        .await
        .unwrap()
        .unwrap_or_default();
    assert!(user.owner == "built-in");
}

#[tokio::test]
async fn test_get_user_with_email() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user_with_email("admin@example.com".to_string())
        .await
        .unwrap()
        .unwrap_or_default();
    assert!(user.email == "admin@example.com");
}

#[tokio::test]
async fn test_add_user() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user("admin".to_string())
        .await
        .unwrap()
        .unwrap_or_default();

    let new_user = CasdoorUser {
        name: "new_user".to_string(),
        ..user
    };

    let res = user_service.add_user(new_user, vec![]).await.unwrap();
    println!("{res:?}");
}

#[tokio::test]
async fn test_update_user() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user("new_user".to_string())
        .await
        .unwrap()
        .unwrap_or_default();

    let new_user = CasdoorUser {
        email: "change@example.com".to_string(),
        ..user
    };

    let res = user_service.update_user(new_user, vec![]).await.unwrap();
    println!("{res:?}");
}

#[tokio::test]
async fn test_delete_user() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user("new_user".to_string())
        .await
        .unwrap()
        .unwrap_or_default();

    let res = user_service.delete_user(user, vec![]).await.unwrap();
    println!("{res:?}");
}
