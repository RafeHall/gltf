use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextureInfo {
    index: u32,
    #[serde(default)]
    tex_coord: u32,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}