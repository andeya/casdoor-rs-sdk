mod models;

use http::Method;
pub use models::*;

use crate::{QueryArgs, Sdk, NONE_BODY};

impl Sdk {
    pub fn enforcer(&self) -> EnforcerSdk {
        EnforcerSdk { sdk: self.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct EnforcerSdk {
    sdk: Sdk,
}

impl EnforcerSdk {
    pub async fn get_enforcer(&self, name: String) -> anyhow::Result<Option<Enforcer>> {
        self.sdk
            .request_data(Method::GET, format!("/api/get-enforcer?id={}", self.sdk.id(&name)), NONE_BODY)
            .await?
            .into_data()
    }
    pub async fn get_enforcers(&self, query_args: QueryArgs) -> anyhow::Result<Vec<Enforcer>> {
        self.sdk
            .request_data(
                Method::GET,
                format!("/api/get-enforcers?{}", self.sdk.get_url_query_part(query_args)?),
                NONE_BODY,
            )
            .await?
            .into_data_default()
    }
    pub async fn enforce(&self, args: EnforceArgs) -> anyhow::Result<bool> {
        let allow_list = self
            .sdk
            .request::<Vec<bool>, Vec<String>>(
                Method::POST,
                format!("/api/enforce?{}", self.sdk.get_url_query_part(args.query)?),
                Some(&args.casbin_request),
            )
            .await?
            .into_data_default()?;
        Ok(allow_list.contains(&true))
    }
    pub async fn batch_enforce(&self, args: BatchEnforceArgs) -> anyhow::Result<Vec<bool>> {
        let allow_lists = self
            .sdk
            .request::<Vec<Vec<bool>>, Vec<String>>(
                Method::POST,
                format!("/api/batch-enforce?{}", self.sdk.get_url_query_part(args.query)?),
                Some(&args.casbin_requests),
            )
            .await?
            .into_data_default()?;
        Ok(allow_lists.into_iter().map(|list| list.contains(&true)).collect())
    }
}
