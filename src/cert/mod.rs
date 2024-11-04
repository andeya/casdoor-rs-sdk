mod models;
pub use models::*;

use crate::{QueryArgs, QueryResult, Sdk, SdkResult};

impl Sdk {
    pub async fn get_certs(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Cert>> {
        self.get_models((), query_args).await
    }
    pub async fn get_global_certs(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Cert>> {
        self.get_models("global", query_args).await
    }
}
