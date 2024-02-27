// TODO: Implement this using custom serialize and deserialization
// and check it using regex ( ^[0-9]+\.[0-9]+$ )
// pub struct GltfVersion {
//     major: usize,
//     minor: usize,
// }

use crate::{extension::Extension, extras::Extras};


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    copyright: Option<String>,
    generator: Option<String>,
    version: String,
    min_version: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}