mod models;

pub use models::*;

use crate::{Body, Method, QueryArgs, QueryResult, Sdk, SdkResult, NO_BODY};

impl Sdk {
    pub async fn get_enforcers(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Enforcer>> {
        self.get_models((), query_args).await
    }
    pub async fn enforce(&self, args: EnforceArgs) -> SdkResult<EnforceResult> {
        let allow_list = self
            .request::<Vec<bool>, Vec<String>>(
                Method::POST,
                self.get_url_path("enforce", true, args.query)?,
                Body::Json(&args.casbin_request),
            )
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
                Body::Json(&args.casbin_requests),
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
            NO_BODY,
        )
        .await?
        .into_data_default()
    }
    pub async fn add_policy(&self, enforcer_name: String, policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("add-policy", false, [("id", self.id(&enforcer_name))])?,
            Body::Json(policy),
        )
        .await?
        .into_data_default()
    }
    pub async fn remove_policy(&self, enforcer_name: String, policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("remove-policy", false, [("id", self.id(&enforcer_name))])?,
            Body::Json(policy),
        )
        .await?
        .into_data_default()
    }
    pub async fn update_policy(&self, enforcer_name: String, old_policy: &CasbinRule, new_policy: &CasbinRule) -> SdkResult<bool> {
        self.request_data(
            Method::POST,
            self.get_url_path("update-policy", false, [("id", self.id(&enforcer_name))])?,
            Body::Json(&[old_policy, new_policy]),
        )
        .await?
        .into_data_default()
    }
    pub async fn get_permissions(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Permission>> {
        self.get_models((), query_args).await
    }
    pub async fn get_permissions_by_submitter(&self) -> SdkResult<QueryResult<Permission>> {
        self.request(Method::GET, self.get_url_path("get-permissions-by-submitter", false, ())?, NO_BODY)
            .await?
            .into_result_default()
            .map(Into::into)
    }
    pub async fn get_permissions_by_role(&self, role_name: &str) -> SdkResult<QueryResult<Permission>> {
        self.request(
            Method::GET,
            self.get_url_path("get-permissions-by-role", false, [("id", self.id(role_name))])?,
            NO_BODY,
        )
        .await?
        .into_result_default()
        .map(Into::into)
    }
    pub async fn get_roles(&self, query_args: QueryArgs) -> SdkResult<QueryResult<Role>> {
        self.get_models((), query_args).await
    }
    pub async fn get_roles_by_user(&self, user_id: &str) -> SdkResult<Vec<String>> {
        self.request_data(Method::GET, self.get_url_path("get-all-roles", false, [("userId", user_id)])?, NO_BODY)
            .await?
            .into_data_default()
    }
}
