use std::num::NonZeroU32;

use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BufferViewTarget {
    /// `GL_ARRAY_BUFFER`
    ArrayBuffer = 34962,
    /// `GL_ELEMENT_ARRAY_BUFFER`
    ElementArrayBuffer = 34963,
}

#[repr(transparent)]
#[derive(serde::Serialize, Debug, Clone)]
pub struct Stride(u8);

impl Stride {
    pub fn new(v: u8) -> Option<Stride> {
        if v >= 4 && v <= 252 {
            Some(Self(v))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl<'de> serde::Deserialize<'de> for Stride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <u8 as serde::Deserialize>::deserialize(deserializer).and_then(|v| {
            if v >= 4 && v <= 252 {
                Ok(Stride(v))
            } else {
                Err(serde::de::Error::custom(format!(
                    "invalid stride, must be >= 4 and <= 252"
                )))
            }
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    buffer: u32,
    #[serde(default)]
    byte_offset: u32,
    byte_length: NonZeroU32,
    byte_stride: Option<Stride>,
    target: Option<BufferViewTarget>,

    name: String,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    uri: Option<String>,
    byte_length: NonZeroU32,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    use super::Stride;

    #[test]
    fn test_stride() {
        check("1", false);
        check("3", false);
        check("4", true);
        check("123", true);
        check("251", true);
        check("252", true);
        check("253", false);
        check("255", false);
    }

    fn check(s: &str, valid: bool) {
        let r = serde_json::from_str::<Stride>(s);
        if valid {
            assert!(r.is_ok());
        } else {
            assert!(r.is_err());
        }
    }
}
