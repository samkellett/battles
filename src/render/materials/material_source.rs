use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct MaterialSource {
    pub name: String,
    pub vertex_shader: PathBuf,
    pub fragment_shader: PathBuf,
    pub texture: String,
}
