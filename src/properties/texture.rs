use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    sampler: Option<u32>,
    source: Option<u32>,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}