use crate::{extension::Extension, extras::Extras};


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    #[serde(flatten)]
    projection: CameraPerspective,
    camera_type: String, // NOTE: Only here to ensure the glTF specification requirement

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CameraPerspective {
    #[serde(rename_all = "camelCase")]
    Orthographic {
        xmag: f32,
        ymag: f32,
        zfar: f32,
        znear: f32,

        extensions: Option<Extension>,
        extras: Option<Extras>,
    },
    #[serde(rename_all = "camelCase")]
    Perspective {
        aspect_ratio: Option<f32>,
        yfov: f32,
        zfar: Option<f32>,
        znear: f32,

        extensions: Option<Extension>,
        extras: Option<Extras>,
    },
}

#[cfg(test)]
mod tests {
    use super::Camera;

    #[test]
    fn camera_test() {
        let src = r#"{
            "orthographic": {
                "xmag": 1.0,
                "ymag": 1.0,
                "zfar": 16.0,
                "znear": 0.01
            },
            "cameraType": "orthographic"
        }"#;

        let result = serde_json::from_str::<Camera>(src).unwrap();

        println!("{:#?}", result);

        let src = r#"{
            "perspective": {
                "aspectRatio": 1.0,
                "yfov": 1.51,
                "znear": 0.01
            },
            "cameraType": "perspective"
        }"#;

        let result = serde_json::from_str::<Camera>(src).unwrap();

        println!("{:#?}", result);
    }
}