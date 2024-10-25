mod errors;
mod models;
use std::{ops::Deref, sync::Arc};

pub use errors::*;
use http::{Method, StatusCode};
pub use models::*;
use serde::{
    de::{DeserializeOwned, Deserializer},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

use crate::{Config, SdkResult};

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
        Data: DeserializeOwned + Default,
        Data2: DeserializeOwned + Default,
    {
        let mut req = reqwest::Client::new()
            .request(method, self.config.endpoint().clone() + url_path.as_ref())
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
        Data: DeserializeOwned + Default,
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
        Data2: DeserializeOwned + Default,
    {
        self.request::<(), Data2>(method, url_path, body).await
    }
    pub async fn modify_model<T: Model>(&self, args: ModelModifyArgs<T>) -> SdkResult<bool> {
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
    pub(crate) fn get_url_query_part(&self, query_args: impl Serialize) -> SdkResult<String> {
        let mut query = format!("owner={}", self.org_name());
        let query_args = serde_urlencoded::to_string(query_args)?;
        if !query_args.is_empty() {
            query = format!("{query}&{}", self.org_name())
        }
        Ok(query)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiResponse<Data, Data2 = ()> {
    pub data: Option<Data>,
    pub data2: Option<Data2>,
    pub name: String,
    #[serde(flatten)]
    pub status: Status,
    pub sub: String,
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
            Status::Err(e) => Err(SdkError::from_str(StatusCode::INTERNAL_SERVER_ERROR, e)),
            Status::Other { status, msg } => Err(SdkError::from_str(
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
        data.ok_or(SdkError::from_str(StatusCode::NOT_FOUND, "Unexpected empty data."))
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
        data2.ok_or(SdkError::from_str(StatusCode::NOT_FOUND, "Unexpected empty data2."))
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
}
