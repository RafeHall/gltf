use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Extension {
    #[serde(flatten)]
    value: Value,
}