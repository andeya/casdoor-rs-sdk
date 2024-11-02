mod models;
pub use models::*;

use crate::{Method, QueryResult, Sdk, SdkResult, NONE_BODY};

impl Sdk {
    pub async fn get_application_by_name(&self, name: String) -> SdkResult<Option<Application>> {
        self.get_model_by_name(name).await
    }

    pub async fn get_user_application(&self, user_name: &str) -> SdkResult<Option<Application>> {
        self.request_data(Method::GET, format!("/api/get-user-application?id={}", self.id(user_name)), NONE_BODY)
            .await?
            .into_data()
    }

    pub async fn get_applications(&self, query_args: ApplicationQueryArgs) -> SdkResult<QueryResult<Application>> {
        self.get_models((), query_args).await
    }

    pub async fn get_organization_applications(&self, query_args: ApplicationQueryArgs) -> SdkResult<QueryResult<Application>> {
        self.get_models("organization", query_args).await
    }
}
