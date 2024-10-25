mod models;

use http::Method;
pub use models::*;

use crate::{QueryArgs, Sdk, SdkResult, NONE_BODY};

impl Sdk {
    pub async fn get_enforcer(&self, name: String) -> SdkResult<Option<Enforcer>> {
        self.request_data(Method::GET, format!("/api/get-enforcer?id={}", self.id(&name)), NONE_BODY)
            .await?
            .into_data()
    }
    pub async fn get_enforcers(&self, query_args: QueryArgs) -> SdkResult<Vec<Enforcer>> {
        self.request_data(
            Method::GET,
            format!("/api/get-enforcers?{}", self.get_url_query_part(query_args)?),
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }
    pub async fn enforce(&self, args: EnforceArgs) -> SdkResult<bool> {
        let allow_list = self
            .request::<Vec<bool>, Vec<String>>(
                Method::POST,
                format!("/api/enforce?{}", self.get_url_query_part(args.query)?),
                Some(&args.casbin_request),
            )
            .await?
            .into_data_default()?;
        Ok(allow_list.contains(&true))
    }
    pub async fn batch_enforce(&self, args: BatchEnforceArgs) -> SdkResult<Vec<bool>> {
        let allow_lists = self
            .request::<Vec<Vec<bool>>, Vec<String>>(
                Method::POST,
                format!("/api/batch-enforce?{}", self.get_url_query_part(args.query)?),
                Some(&args.casbin_requests),
            )
            .await?
            .into_data_default()?;
        Ok(allow_lists.into_iter().map(|list| list.contains(&true)).collect())
    }
}
