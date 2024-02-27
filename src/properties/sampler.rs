use crate::{extension::Extension, extras::Extras};


#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum TextureMagFiltering {
    /// `GL_NEAREST`
    Nearest = 9728,
    /// `GL_LINEAR`
    Linear = 9729,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum TextureMinFiltering {
    /// `GL_NEAREST`
    Nearest = 9728,
    /// `GL_LINEAR`
    Linear = 9729,
    /// `GL_NEAREST_MIPMAP_NEAREST`
    NearestMipmapNearest = 9984,
    /// `GL_LINEAR_MIPMAP_NEAREST`
    LinearMipmapNearest = 9985,
    /// `GL_NEAREST_MIPMAP_LINEAR`
    NearestMipmapLinear = 9986,
    /// `GL_LINEAR_MIPMAP_LINEAR`
    LinearMipmapLinear = 9987,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum TextureWrapping {
    /// `GL_CLAMP_TO_EDGE`
    ClampToEdge = 33071,
    /// `GL_MIRRORED_REPEAT`
    MirroredRepeat = 33648,
    /// `GL_REPEAT`
    #[default]
    Repeat = 10497,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    mag_filter: Option<TextureMagFiltering>,
    min_filter: Option<TextureMinFiltering>,
    #[serde(default)]
    wrap_s: TextureWrapping,
    #[serde(default)]
    wrap_t: TextureWrapping,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}