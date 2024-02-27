use vec1::Vec1;

use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    inverse_bind_matrices: Option<u32>,
    skeleton: Option<u32>,
    joints: Vec1<u32>,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}