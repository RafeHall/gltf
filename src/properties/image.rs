use crate::{extension::Extension, extras::Extras};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ImageSource {
    #[serde(rename_all = "camelCase")]
    Uri {
        uri: String,
        mime_type: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Buffer {
        buffer_view: u32,
        mime_type: String,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(flatten)]
    image_source: ImageSource,

    name: Option<String>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn test_source() {
        let src = r#"
            {
                "uri": "path/to/image.png",
                "mimeType": "image/png"
            }
        "#;

        let result = serde_json::from_str::<Image>(src).unwrap();

        println!("{:#?}", result);

        let src = r#"
            {
                "uri": "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png",
                "name": "google_logo",
                "extensions": [
                    "idk, https download or something ext."
                ]
            }
        "#;

        let result = serde_json::from_str::<Image>(src).unwrap();

        println!("{:#?}", result);

        let src = r#"
            {
                "bufferView": 0,
                "mimeType": "image/jpeg"
            }
        "#;

        let result = serde_json::from_str::<Image>(src).unwrap();

        println!("{:#?}", result);
    }
}