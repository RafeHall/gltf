use std::num::NonZeroU32;

use crate::{accessor_sparse::AccessorSparse, extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ComponentRange {
    Scalar([f32; 1]),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4Mat4([f32; 4]),
    Mat3([f32; 9]),
    Mat4([f32; 16]),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum AccessorType {
    /// `GL_FLOAT`
    Scalar = 5126,
    /// `GL_FLOAT_VEC2`
    Vec2 = 35664,
    /// `GL_FLOAT_VEC3`
    Vec3 = 35665,
    /// `GL_FLOAT_VEC4`
    Vec4 = 35666,
    /// `GL_FLOAT_MAT2`
    Mat2 = 35674,
    /// `GL_FLOAT_MAT3`
    Mat3 = 35675,
    /// `GL_FLOAT_MAT4`
    Mat4 = 35676,
}

/// Represents the OpenGL component type that an [`Accessor`] value represents
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
pub enum ComponentType {
    /// `GL_BYTE`
    Byte = 5120,
    /// `GL_UNSIGNED_BYTE`
    UnsignedByte = 5121,
    /// `GL_SHORT`
    Short = 5122,
    /// `GL_UNSIGNED_SHORT`
    UnsignedShort = 5123,
    /// `GL_UNSIGNED_INT`
    UnsignedInt = 5125,
    /// `GL_FLOAT`
    Float = 5126,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    buffer_view: Option<u32>,
    #[serde(default)]
    byte_offset: u32,
    component_type: ComponentType,
    #[serde(default)]
    normalized: bool,
    count: NonZeroU32,
    #[serde(rename = "type")]
    accessor_type: AccessorType,
    max: Option<ComponentRange>,
    min: Option<ComponentRange>,
    sparse: Option<AccessorSparse>,
    
    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::accessor::ComponentRange;

    #[test]
    fn test_component_range() {
        #[rustfmt::skip]
        const VALID: [bool; 17] = [
            false,
            true, true, true, true,
            false, false, false, false,
            true, false, false, false,
            false, false, false, true,
        ];

        fn generate_count(count: usize) -> String {
            let mut string = String::new();

            for i in 0..count {
                let num: f32 = rand::random();
                write!(&mut string, "{}", num).unwrap();
                if i != count - 1 {
                    write!(&mut string, ", ").unwrap();
                }
            }

            format!("[{}]", string)
        }

        for i in 0..17 {
            let string = generate_count(i);
            let component_range = serde_json::from_str::<ComponentRange>(&string);

            if VALID[i] {
                assert!(
                    component_range.is_ok(),
                    "Invalid range element count, should be ok: {i}"
                );
            } else {
                assert!(
                    component_range.is_err(),
                    "Invalid range element count result, should be error: {i}"
                );
            }
        }
    }
}
