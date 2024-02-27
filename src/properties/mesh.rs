use serde_json::{Map, Value};
use vec1::Vec1;

use crate::{extension::Extension, extras::Extras};

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum PrimitiveMode {
    /// `GL_POINTS`
    Points = 0,
    /// `GL_LINES`
    Lines = 1,
    /// `GL_LINE_LOOP`
    LineLoop = 2,
    /// `GL_LINE_STRIP`
    LineStrip = 3,
    /// `GL_TRIANGLES`
    #[default]
    Triangles = 4,
    /// `GL_TRIANGLE_STRIP`
    TriangleStrip = 5,
    /// `GL_TRIANGLE_FAN`
    TriangleFan = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    attributes: Map<String, Value>,
    indices: Option<u32>,
    material: Option<u32>,
    #[serde(default)]
    mode: PrimitiveMode,
    #[serde(default)]
    targets: Vec<Map<String, Value>>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    primitives: Vec1<Primitive>,
    #[serde(default)]
    weights: Vec<f32>,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}