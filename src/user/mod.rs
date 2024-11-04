mod models;

pub use models::*;

use crate::{Method, QueryResult, Sdk, SdkError, SdkResult, StatusCode, NONE_BODY};

impl Sdk {
    /// Query and return some models and the total number of models.
    pub async fn get_users(&self, query_args: UserQueryArgs) -> SdkResult<QueryResult<User>> {
        self.get_models((), query_args).await
    }

    pub async fn get_user_groups(&self, query_args: UserGroupQueryArgs) -> SdkResult<QueryResult<UserGroup>> {
        self.get_models((), query_args).await
    }

    pub async fn get_user_count(&self, is_online: QueryUserSet) -> SdkResult<i64> {
        self.request_data(
            Method::GET,
            self.get_url_path("get-user-count", true, [("isOnline", is_online.to_string())])?,
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }

    pub async fn get_user(&self, args: GetUserArgs) -> SdkResult<Option<User>> {
        match (args.user_id, args.name, args.email, args.phone) {
            (Some(user_id), None, None, None) => self.get_user_by_user_id(user_id).await,
            (None, Some(name), None, None) => self.get_user_by_name(name).await,
            (None, None, Some(email), None) => self.get_user_by_email(email).await,
            (None, None, None, Some(phone)) => self.get_user_by_phone(phone).await,
            _ => Err(SdkError::new(
                StatusCode::BAD_REQUEST,
                r##"The parameters "uid", "name", "email" and "phone" can and must only pass one."##,
            )),
        }
    }

    pub async fn get_user_by_name(&self, name: String) -> SdkResult<Option<User>> {
        self.get_model_by_name(name).await
    }

    pub async fn get_user_group_by_name(&self, name: String) -> SdkResult<Option<UserGroup>> {
        self.get_model_by_name(name).await
    }

    pub async fn get_user_by_email(&self, email: String) -> SdkResult<Option<User>> {
        self.request_data(Method::GET, format!("/api/get-user?owner={}&email={}", self.org_name(), email), NONE_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> SdkResult<Option<User>> {
        self.request_data(Method::GET, format!("/api/get-user?owner={}&phone={}", self.org_name(), phone), NONE_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_user_id(&self, user_id: String) -> SdkResult<Option<User>> {
        self.request_data(
            Method::GET,
            format!("/api/get-user?owner={}&userId={}", self.org_name(), user_id),
            NONE_BODY,
        )
        .await?
        .into_data()
    }
}
