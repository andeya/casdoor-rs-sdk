mod models;
pub use models::*;

use crate::{QueryArgs, QueryResult, Sdk, SdkResult};

impl Sdk {
    pub async fn get_providers(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Provider>> {
        self.get_models((), query_args).await
    }
    pub async fn get_global_providers(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Provider>> {
        self.get_models("global", query_args).await
    }
}
