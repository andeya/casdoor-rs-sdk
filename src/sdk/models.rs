use std::fmt::{Debug, Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(not(feature = "salvo"))]
pub trait Model: Debug + Clone + PartialEq + Eq + DeserializeOwned + Serialize {
    /// Model identifier, used for splicing URLs.
    fn ident() -> &'static str;
    /// Indicate whether this model currently supports updating individual columns one by one.
    fn support_update_columns() -> bool;
    fn owner(&self) -> &str;
    fn name(&self) -> &str;
    fn id(&self) -> String {
        format!("{}/{}", self.owner(), self.name())
    }
}
#[cfg(feature = "salvo")]
pub trait Model: Debug + Clone + PartialEq + Eq + DeserializeOwned + Serialize + salvo::prelude::ToSchema {
    /// Model identifier, used for splicing URLs.
    fn ident() -> &'static str;
    /// Models identifier, used for splicing URLs.
    fn plural_ident() -> &'static str;
    /// Indicate whether this model currently supports updating individual columns one by one.
    fn support_update_columns() -> bool;
    fn owner(&self) -> &str;
    fn name(&self) -> &str;
    fn id(&self) -> String {
        format!("{}/{}", self.owner(), self.name())
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelModifyArgs<M> {
    pub action: ModelAction,
    pub model: M,
    pub columns: Option<Vec<String>>,
}
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelAddArgs<M> {
    pub model: M,
}
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelUpdateArgs<M> {
    pub model: M,
    pub columns: Vec<String>,
}
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelDeleteArgs<M> {
    pub model: M,
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ModelAction {
    Add,
    Delete,
    Update,
}

impl Display for ModelAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelAction::Add => write!(f, "add"),
            ModelAction::Delete => write!(f, "delete"),
            ModelAction::Update => write!(f, "update"),
        }
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelActionAffect {
    #[default]
    Affected,
    Unaffected,
}

impl Display for ModelActionAffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Affected => write!(f, "Affected"),
            Self::Unaffected => write!(f, "Unaffected"),
        }
    }
}

impl ModelActionAffect {
    pub fn is_affected(&self) -> bool {
        matches!(self, ModelActionAffect::Affected)
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToParameters, salvo::prelude::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct QueryArgs {
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "sortField", skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[cfg_attr(feature = "salvo", salvo(parameter(parameter_in=Query)))]
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

pub(crate) trait IsQueryArgs: Serialize {}

impl IsQueryArgs for QueryArgs {}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult<M> {
    items: Vec<M>,
    total: i64,
}

impl<M> From<(Vec<M>, i64)> for QueryResult<M> {
    #[inline(always)]
    fn from(value: (Vec<M>, i64)) -> Self {
        Self {
            items: value.0,
            total: value.1,
        }
    }
}
