use std::num::NonZeroU32;

use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentType {
    /// `GL_UNSIGNED_BYTE`
    #[default]
    UnsignedByte = 5121,
    /// `GL_UNSIGNED_SHORT`
    UnsignedShort = 5123,
    /// `GL_UNSIGNED_INT`
    UnsignedInt = 5125,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessorSparseIndices {
    buffer_view: u32,
    #[serde(default)]
    byte_offset: u32,
    component_type: ComponentType,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessorSparseValues {
    buffer_view: u32,
    #[serde(default)]
    byte_offset: u32,

    extensions: Option<Extension>,
    extra: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessorSparse {
    count: NonZeroU32,
    indices: AccessorSparseIndices,
    values: AccessorSparseValues,
    
    extensions: Option<Extension>,
    extras: Option<Extras>,
}