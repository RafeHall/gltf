use crate::{extension::Extension, extras::Extras};

#[rustfmt::skip]
const fn identity_matrix() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

const fn identity_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

const fn identity_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

const fn identity_translation() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    camera: Option<u32>,
    #[serde(default)]
    children: Vec<u32>,
    skin: Option<u32>,
    #[serde(default = "identity_matrix")]
    matrix: [f32; 16],
    mesh: Option<u32>,
    #[serde(default = "identity_rotation")]
    rotation: [f32; 4],
    #[serde(default = "identity_scale")]
    scale: [f32; 3],
    #[serde(default = "identity_translation")]
    translation: [f32; 3],
    #[serde(default)]
    weights: Vec<f32>,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}