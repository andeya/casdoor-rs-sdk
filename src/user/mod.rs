mod models;

use http::Method;
pub use models::*;

use crate::{Sdk, NONE_BODY};

impl Sdk {
    pub fn user(&self) -> UserSdk {
        UserSdk { sdk: self.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct UserSdk {
    sdk: Sdk,
}

impl UserSdk {
    pub async fn get_users(&self, query_args: UserQueryArgs) -> anyhow::Result<Vec<User>> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-users?{}", self.sdk.get_url_query_part(query_args)?),
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn get_user_count(&self, is_online: QueryUserSet) -> anyhow::Result<i64> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-user-count?owner={}&isOnline={}", self.sdk.org_name(), is_online),
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }

    pub async fn get_user(&self, name: String) -> anyhow::Result<Option<User>> {
        self.sdk
            .request_data(Method::GET, format!("/api/get-user?id={}", self.sdk.id(&name)), NONE_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<User>> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&email={}", self.sdk.org_name(), email),
                NONE_BODY,
            )
            .await?
            .into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> anyhow::Result<Option<User>> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&phone={}", self.sdk.org_name(), phone),
                NONE_BODY,
            )
            .await?
            .into_data()
    }

    pub async fn get_user_by_user_id(&self, user_id: String) -> anyhow::Result<Option<User>> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-user?owner={}&userId={}", self.sdk.org_name(), user_id),
                NONE_BODY,
            )
            .await?
            .into_data()
    }
}
