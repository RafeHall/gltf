use crate::{accessor::Accessor, animation::Animation, asset::Asset, buffer::{Buffer, BufferView}, camera::Camera, extension::Extension, extras::Extras, image::Image, material::Material, mesh::Mesh, node::Node, sampler::Sampler, scene::Scene, skin::Skin, texture::Texture};


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    #[serde(default)]
    extensions_used: Vec<String>,
    #[serde(default)]
    extensions_required: Vec<String>,
    #[serde(default)]
    accessors: Vec<Accessor>,
    #[serde(default)]
    animations: Vec<Animation>,
    asset: Asset,
    #[serde(default)]
    buffers: Vec<Buffer>,
    #[serde(default)]
    buffer_views: Vec<BufferView>,
    #[serde(default)]
    cameras: Vec<Camera>,
    #[serde(default)]
    images: Vec<Image>,
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    meshes: Vec<Mesh>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    samplers: Vec<Sampler>,
    scene: Option<u32>,
    #[serde(default)]
    scenes: Vec<Scene>,
    #[serde(default)]
    skins: Vec<Skin>,
    #[serde(default)]
    textures: Vec<Texture>,

    extensions: Option<Extension>,
    extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    use super::Gltf;

    #[test]
    fn test_succeed() {
        let paths = glob::glob("example_models/succeed/**/*.gltf").unwrap();

        for path in paths {
            let path = path.unwrap();

            println!("Loading File @ {}", path.clone().to_string_lossy());

            let r = std::fs::File::open(path.clone()).unwrap();
            let _result: Gltf = serde_json::from_reader(r).unwrap();

            // println!("{:#?}", result);
        }
    }

    #[test]
    fn test_fail() {
        let paths = glob::glob("example_models/fail/**/*.gltf").unwrap();

        for path in paths {
            let path = path.unwrap();

            println!("Loading File @ {}", path.clone().to_string_lossy());

            let _r = std::fs::File::open(path.clone()).unwrap();
            // TODO: Validation so this properly fails
            // assert!(serde_json::from_reader::<_, Gltf>(r).is_err(), "Should fail");

            // println!("{:#?}", result);
        }
    }
}