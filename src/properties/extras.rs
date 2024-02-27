use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Extras {
    #[serde(flatten)]
    value: Value,
}