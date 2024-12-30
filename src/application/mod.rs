mod models;
pub use models::*;

use crate::{Method, NO_BODY, QueryResult, Sdk, SdkResult};

impl Sdk {
    pub async fn get_user_application(&self, user_name: &str) -> SdkResult<Option<Application>> {
        self.request_data(
            Method::GET,
            format!("/api/get-user-application?id={}", self.id(user_name)),
            NO_BODY,
        )
        .await?
        .into_data()
    }

    pub async fn get_applications(&self, query_args: ApplicationQueryArgs) -> SdkResult<QueryResult<Application>> {
        self.get_models((), query_args).await
    }

    pub async fn get_organization_applications(
        &self,
        query_args: ApplicationQueryArgs,
    ) -> SdkResult<QueryResult<Application>> {
        self.get_models("organization", query_args).await
    }
}
