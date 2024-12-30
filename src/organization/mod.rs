mod models;
pub use models::*;

use crate::{Method, NO_BODY, QueryResult, Sdk, SdkResult};

impl Sdk {
    pub async fn get_default_organization(&self, name: String) -> SdkResult<Option<Organization>> {
        self.get_default_model(name).await
    }
    pub async fn get_organizations(&self, query_args: OrganizationQueryArgs) -> SdkResult<QueryResult<Organization>> {
        self.get_models((), query_args).await
    }
    /// NOTE: Only obtain fields `name` and `display_name` of `Organization`.
    pub async fn get_organization_names(&self) -> SdkResult<Vec<Organization>> {
        self.request_data(
            Method::GET,
            self.get_url_path("get-organization-names", true, ())?,
            NO_BODY,
        )
        .await?
        .into_data_default()
    }
}
