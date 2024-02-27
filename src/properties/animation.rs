use vec1::Vec1;

use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationChannelTarget {
    node: Option<u32>,
    path: String,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationChannel {
    sampler: u32,
    target: AnimationChannelTarget,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnimationInterpolation {
    #[default]
    Linear,
    Step,
    Cubicspline,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationSampler {
    input: u32,
    #[serde(default)]
    interpolation: AnimationInterpolation,
    output: u32,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    channels: Vec1<AnimationChannel>,
    samplers: Vec1<AnimationSampler>,
    
    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}