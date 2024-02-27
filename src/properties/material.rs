use crate::{extension::Extension, extras::Extras, texture_info::TextureInfo};

const fn vec4_one() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

const fn half() -> f32 {
    0.5
}

const fn one() -> f32 {
    1.0
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlphaMode {
    #[default]
    Opaque,
    Mask,
    Blend,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OcclusionTextureInfo {
    index: u32,
    #[serde(default)]
    tex_coord: u32,
    #[serde(default = "one")]
    strength: f32,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NormalTextureInfo {
    index: u32,
    #[serde(default)]
    tex_coord: u32,
    #[serde(default = "one")]
    scale: f32,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    #[serde(default = "vec4_one")]
    base_color_factor: [f32; 4],
    base_color_texture: Option<TextureInfo>,
    #[serde(default = "one")]
    metallic_factor: f32,
    #[serde(default = "one")]
    roughness_factor: f32,
    metallic_roughness_texture: Option<TextureInfo>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    #[serde(rename = "pbrMetallicRoughness")]
    pbr: Option<PbrMetallicRoughness>,
    normal_texture: Option<NormalTextureInfo>,
    occlusion_texture: Option<OcclusionTextureInfo>,
    emissive_texture: Option<TextureInfo>,
    #[serde(default)]
    emissive_factor: [f32; 3],
    #[serde(default)]
    alpha_mode: AlphaMode,
    #[serde(default = "half")]
    alpha_cutoff: f32,
    #[serde(default)]
    double_sided: bool,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}