mod models;

pub use models::*;

use crate::{Body, Method, QueryResult, Sdk, SdkError, SdkResult, StatusCode, NO_BODY};

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
            NO_BODY,
        )
        .await?
        .into_data_default()
    }

    pub async fn get_user(&self, args: GetUserArgs) -> SdkResult<Option<User>> {
        match (args.user_id, args.name, args.email, args.phone) {
            (Some(user_id), None, None, None) => self.get_user_by_user_id(user_id).await,
            (None, Some(name), None, None) => self.get_model_by_name(name).await,
            (None, None, Some(email), None) => self.get_user_by_email(email).await,
            (None, None, None, Some(phone)) => self.get_user_by_phone(phone).await,
            _ => Err(SdkError::new(
                StatusCode::BAD_REQUEST,
                r##"The parameters "uid", "name", "email" and "phone" can and must only pass one."##,
            )),
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> SdkResult<Option<User>> {
        self.request_data(Method::GET, format!("/api/get-user?owner={}&email={}", self.org_name(), email), NO_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_phone(&self, phone: String) -> SdkResult<Option<User>> {
        self.request_data(Method::GET, format!("/api/get-user?owner={}&phone={}", self.org_name(), phone), NO_BODY)
            .await?
            .into_data()
    }

    pub async fn get_user_by_user_id(&self, user_id: String) -> SdkResult<Option<User>> {
        self.request_data(
            Method::GET,
            format!("/api/get-user?owner={}&userId={}", self.org_name(), user_id),
            NO_BODY,
        )
        .await?
        .into_data()
    }

    /// NOTE: oldPassword is not required, if you don't need, just pass a empty string
    pub async fn set_user_password(&self, mut args: SetPasswordArgs) -> SdkResult<()> {
        args.user_owner.get_or_insert(self.org_name().to_owned());
        self.request_data(Method::POST, "/api/set-password", Body::Form(&serde_urlencoded::to_string(args)?))
            .await?
            .into_data_default()
    }
}
