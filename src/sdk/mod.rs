mod errors;
mod models;
use std::{ops::Deref, sync::Arc};

use cubix::MaybeString;
pub use errors::*;
pub use models::*;
use serde::{
    de::{DeserializeOwned, Deserializer},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

use crate::{Config, Method, SdkResult, StatusCode};

#[derive(Debug, Clone)]
pub struct Sdk {
    config: Arc<Config>,
}
impl Deref for Sdk {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Config {
    pub fn into_sdk(self) -> Sdk {
        Sdk::new(self)
    }
}

pub const NONE_BODY: Option<&()> = None::<&()>;

impl Sdk {
    pub fn new(config: Config) -> Self {
        Self { config: Arc::new(config) }
    }
    pub fn id(&self, name: &str) -> String {
        format!("{}/{}", self.org_name(), name)
    }
    pub async fn request<Data, Data2>(
        &self,
        method: Method,
        url_path: impl AsRef<str>,
        body: Option<&impl Serialize>,
    ) -> SdkResult<ApiResponse<Data, Data2>>
    where
        Data: DeserializeOwned,
        Data2: DeserializeOwned,
    {
        let url = self.config.endpoint().clone() + url_path.as_ref();
        println!("{url}");
        let mut req = reqwest::Client::new()
            .request(method, url)
            .basic_auth(self.config.client_id().clone(), Some(self.config.client_secret().clone()));
        if let Some(body) = body {
            req = req.json(body);
        }
        Ok(req.send().await?.json::<ApiResponse<Data, Data2>>().await?)
    }
    pub async fn request_data<Data>(
        &self,
        method: Method,
        url_path: impl AsRef<str>,
        body: Option<&impl Serialize>,
    ) -> SdkResult<ApiResponse<Data, ()>>
    where
        Data: DeserializeOwned,
    {
        self.request::<Data, ()>(method, url_path, body).await
    }
    pub async fn request_data2<Data2>(
        &self,
        method: Method,
        url_path: impl AsRef<str>,
        body: Option<&impl Serialize>,
    ) -> SdkResult<ApiResponse<(), Data2>>
    where
        Data2: DeserializeOwned,
    {
        self.request::<(), Data2>(method, url_path, body).await
    }
    pub async fn modify_model<T: Model>(&self, args: ModelModifyArgs<T>) -> SdkResult<bool> {
        // When adding model, the id parameter is not needed.
        let mut url_path = format!("/api/{}-{}?id={}", args.action, T::ident(), args.model.id());
        if matches!(args.action, ModelAction::Update) && args.columns.is_some() {
            let columns = args.columns.unwrap();
            if !columns.is_empty() {
                url_path += &format!("&columns={}", columns.join(","));
            }
        }
        self.request_data::<ModelActionAffect>(Method::POST, url_path, Some(&args.model))
            .await?
            .into_data_default()
            .map(|v| v.is_affected())
    }
    pub async fn add_model<T: Model>(&self, args: ModelAddArgs<T>) -> SdkResult<bool> {
        self.modify_model(ModelModifyArgs {
            action: ModelAction::Add,
            model: args.model,
            columns: None,
        })
        .await
    }
    pub async fn update_model<T: Model>(&self, args: ModelUpdateArgs<T>) -> SdkResult<bool> {
        self.modify_model(ModelModifyArgs {
            action: ModelAction::Update,
            model: args.model,
            columns: Some(args.columns),
        })
        .await
    }
    pub async fn delete_model<T: Model>(&self, args: ModelDeleteArgs<T>) -> SdkResult<bool> {
        self.modify_model(ModelModifyArgs {
            action: ModelAction::Delete,
            model: args.model,
            columns: None,
        })
        .await
    }
    pub async fn get_model_by_name<M: Model>(&self, name: String) -> Result<Option<M>, SdkError> {
        self.request_data(Method::GET, format!("/api/get-{}?id={}", M::ident(), self.id(&name)), NONE_BODY)
            .await?
            .into_data()
    }
    pub async fn get_default_model<M: Model>(&self, name: String) -> Result<Option<M>, SdkError> {
        self.request_data(Method::GET, format!("/api/get-default-{}?id={}", M::ident(), self.id(&name)), NONE_BODY)
            .await?
            .into_data()
    }
    // Query and return some models and the total number of models.
    pub(crate) async fn get_models<M: Model>(&self, mid_ident: impl Into<MaybeString>, query_args: impl IsQueryArgs) -> SdkResult<QueryResult<M>> {
        let ident = if let Some(mid) = mid_ident.into().option_string() {
            "get-".to_owned() + &mid + "-"
        } else {
            "get-".to_owned()
        } + M::plural_ident();

        self.request(Method::GET, self.get_url_path(ident, true, query_args)?, NONE_BODY)
            .await?
            .into_result_default()
            .map(Into::into)
    }
    pub(crate) fn get_url_path(&self, ident: impl Into<String>, add_owner_query: bool, query_args: impl Serialize) -> SdkResult<String> {
        Ok(format!("/api/{}?{}", ident.into(), self.get_url_query_part(add_owner_query, query_args)?))
    }
    pub(crate) fn get_url_query_part(&self, add_owner_query: bool, query_args: impl Serialize) -> SdkResult<String> {
        let mut query = if add_owner_query {
            format!("owner={}", self.org_name())
        } else {
            String::new()
        };
        let query_args = serde_urlencoded::to_string(query_args)?;
        if !query_args.is_empty() {
            query = format!("{query}&{query_args}")
        }
        Ok(query)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiResponse<Data, Data2 = ()> {
    pub data: Option<Data>,
    pub data2: Option<Data2>,
    pub name: String,
    #[serde(flatten)]
    pub status: Status,
    pub sub: String,
}

impl<Data, Data2> Default for ApiResponse<Data, Data2> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            data2: Default::default(),
            name: Default::default(),
            status: Default::default(),
            sub: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Ok(String),
    Err(String),
    Other { status: String, msg: String },
}

impl Default for Status {
    fn default() -> Self {
        Self::Ok(Default::default())
    }
}
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StatusContainer {
            status: String,
            msg: String,
        }

        let container = StatusContainer::deserialize(deserializer)?;
        match container.status.as_str() {
            "ok" => Ok(Status::Ok(container.msg)),
            "error" => Ok(Status::Err(container.msg)),
            _ => Ok(Status::Other {
                status: container.status,
                msg: container.msg,
            }),
        }
    }
}
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create a new struct to serialize the enum variants
        let mut state = serializer.serialize_struct("Status", 2)?;
        match self {
            Status::Ok(msg) => {
                state.serialize_field("status", "ok")?;
                state.serialize_field("msg", msg)?;
            }
            Status::Err(msg) => {
                state.serialize_field("status", "error")?;
                state.serialize_field("msg", msg)?;
            }
            Status::Other { status, msg } => {
                state.serialize_field("status", status)?;
                state.serialize_field("msg", msg)?;
            }
        }
        state.end()
    }
}

impl<Data, Data2> ApiResponse<Data, Data2> {
    pub fn into_result(self) -> SdkResult<(Option<Data>, Option<Data2>)> {
        match self.status {
            Status::Ok(_) => Ok((self.data, self.data2)),
            Status::Err(e) => Err(SdkError::new(StatusCode::INTERNAL_SERVER_ERROR, e)),
            Status::Other { status, msg } => Err(SdkError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown: status={status}, msg={msg}"),
            )),
        }
    }

    pub fn into_result_default(self) -> SdkResult<(Data, Data2)>
    where
        Data: Default,
        Data2: Default,
    {
        let (data, data2) = self.into_result()?;
        Ok((data.unwrap_or_default(), data2.unwrap_or_default()))
    }

    pub fn into_data(self) -> SdkResult<Option<Data>> {
        let (data, _) = self.into_result()?;
        Ok(data)
    }

    pub fn into_data_value(self) -> SdkResult<Data> {
        let (data, _) = self.into_result()?;
        data.ok_or(SdkError::new(StatusCode::NOT_FOUND, "Unexpected empty data."))
    }

    pub fn into_data_default(self) -> SdkResult<Data>
    where
        Data: Default,
    {
        let (data, _) = self.into_result()?;
        Ok(data.unwrap_or_default())
    }

    pub fn into_data2(self) -> SdkResult<Option<Data2>> {
        let (_, data2) = self.into_result()?;
        Ok(data2)
    }

    pub fn into_data2_value(self) -> SdkResult<Data2> {
        let (_, data2) = self.into_result()?;
        data2.ok_or(SdkError::new(StatusCode::NOT_FOUND, "Unexpected empty data2."))
    }

    pub fn into_data2_default(self) -> SdkResult<Data2>
    where
        Data2: Default,
    {
        let (_, data2) = self.into_result()?;
        Ok(data2.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_res_json() {
        let json_data = r#"{"data":{"accessKey":"test"},"data2":null,"name":"","status":"ok","msg":"test","sub":""}"#;
        let obj: ApiResponse<HashMap<String, String>, ()> = serde_json::from_str(&json_data).unwrap();
        println!("{obj:?}");
        let json_data2 = serde_json::to_string(&obj).unwrap();
        assert_eq!(json_data, json_data2);
    }
    #[test]
    fn test_query_url() {
        let query = serde_urlencoded::to_string([
            ("bread", "ba/guette".to_owned()),
            ("cheese", "comté".to_owned()),
            ("meat", "ham".to_owned()),
            ("fat", "butter".to_owned()),
        ])
        .unwrap();
        assert_eq!("bread=ba%2Fguette&cheese=comt%C3%A9&meat=ham&fat=butter", query);
        assert_eq!("", serde_urlencoded::to_string(()).unwrap());
        assert_eq!("", serde_urlencoded::to_string(Vec::<()>::new()).unwrap());
    }
    #[test]
    #[should_panic]
    fn test_query_url4() {
        let _ = serde_urlencoded::to_string(("k", "v")).unwrap();
    }
}
