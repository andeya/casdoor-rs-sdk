mod models;

pub use models::*;

use crate::{Method, QueryArgs, QueryResult, Sdk, SdkResult, NONE_BODY};

impl Sdk {
    pub async fn get_enforcer(&self, name: String) -> SdkResult<Option<Enforcer>> {
        self.get_model_by_name(name).await
    }
    pub async fn get_enforcers(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Enforcer>> {
        self.get_models((), query_args).await
    }
    pub async fn enforce(&self, args: EnforceArgs) -> SdkResult<EnforceResult> {
        let allow_list = self
            .request::<Vec<bool>, Vec<String>>(Method::POST, self.get_url_path("enforce", true, args.query)?, Some(&args.casbin_request))
            .await?
            .into_data_default()?;
        Ok(EnforceResult {
            allow: allow_list.contains(&true),
        })
    }
    pub async fn batch_enforce(&self, args: BatchEnforceArgs) -> SdkResult<BatchEnforceResult> {
        let allow_lists = self
            .request::<Vec<Vec<bool>>, Vec<String>>(
                Method::POST,
                self.get_url_path("batch-enforce", true, args.query)?,
                Some(&args.casbin_requests),
            )
            .await?
            .into_data_default()?;
        Ok(BatchEnforceResult {
            allow_list: allow_lists.into_iter().map(|list| list.contains(&true)).collect(),
        })
    }
    pub async fn get_policies(&self, enforcer_name: String) -> SdkResult<Vec<CasbinRule>> {
        self.request_data(
            Method::GET,
            self.get_url_path("get-policies", false, [("id", self.id(&enforcer_name))])?,
            NONE_BODY,
        )
        .await?
        .into_data_default()
    }
    pub async fn add_policy(&self, enforcer_name: String, policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("add-policy", false, [("id", self.id(&enforcer_name))])?,
            Some(policy),
        )
        .await?
        .into_data_default()
    }
    pub async fn remove_policy(&self, enforcer_name: String, policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("remove-policy", false, [("id", self.id(&enforcer_name))])?,
            Some(policy),
        )
        .await?
        .into_data_default()
    }
    pub async fn update_policy(&self, enforcer_name: String, old_policy: &CasbinRule, new_policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("update-policy", false, [("id", self.id(&enforcer_name))])?,
            Some(&[old_policy, new_policy]),
        )
        .await?
        .into_data_default()
    }
    pub async fn get_all_roles(&self, user_id: &str) -> SdkResult<Vec<String>> {
        self.request_data(Method::GET, self.get_url_path("get-all-roles", false, [("userId", user_id)])?, NONE_BODY)
            .await?
            .into_data_default()
    }
}
