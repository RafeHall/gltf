use crate::{extension::Extension, extras::Extras};


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    nodes: Vec<u32>,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}