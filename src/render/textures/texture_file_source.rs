use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SliceFileSource {
    pub name: String,
    pub origin: [i32; 2],
    pub dimensions: [i32; 2],
}

#[derive(Debug, Deserialize)]
pub struct TextureFileSource {
    pub texture_file: PathBuf,
    pub slices: Vec<SliceFileSource>,
}

