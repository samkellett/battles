extern crate cgmath;
extern crate glium;
extern crate image;

use std::collections::HashMap;

// Internal representation of a texture in a texture collection.
struct TextureSlice {
    // An index that points to the texture.
    id: usize,
    // The top-left corner of the texture.
    origin: cgmath::Vector2<i32>,
    // The width and height of the texture.
    dimensions: cgmath::Vector2<i32>,
}

// A collection of textures and mappings onto each one.
pub struct TextureCollection {
    // All of the textures available.
    textures: Vec<glium::texture::Texture2d>,
    // Maps of texture by name to it's source file and where in the file it is.
    views: HashMap<String, TextureSlice>,
}

// A reference to a texture in the collection.
#[derive(Debug)]
pub struct Texture<'a> {
    pub texture: &'a glium::texture::Texture2d,
    pub origin: &'a cgmath::Vector2<i32>,
    pub dimensions: &'a cgmath::Vector2<i32>,
}

struct SliceSource {
    pub name: String,
    pub origin: cgmath::Vector2<i32>,
    pub dimensions: cgmath::Vector2<i32>,
}

struct TextureSource {
    pub texture_file: std::path::PathBuf,
    pub texture_format: image::ImageFormat,
    pub slices: Vec<SliceSource>,
}

impl TextureCollection {
    fn new<D, I>(display: &D, sources: I) -> TextureCollection
        where D: glium::backend::Facade,
              I: Iterator<Item=TextureSource>,
    {
        let mut textures = Vec::new();
        let mut views = HashMap::new();

        for source in sources {
            // Load the texture.
            let image = {
                let file = std::fs::File::open(source.texture_file).unwrap();
                let file = std::io::BufReader::new(file);

                image::load(file, source.texture_format)
                    .unwrap()
                    .to_rgba()
            };

            let image_dimensions = image.dimensions();
            let texture = {
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                               image_dimensions);
                glium::texture::Texture2d::new(display, image).unwrap()
            };

            let next_id = textures.len();
            textures.push(texture);

            for slice in source.slices {
                // Build the slice.
                let view = TextureSlice {
                    id: next_id,
                    origin: slice.origin,
                    dimensions: slice.dimensions,
                };
                views.insert(slice.name.to_owned(), view);
            }
        }

        TextureCollection { textures: textures, views: views }
    }

    // Get a reference to a registered material.
    fn texture(&self, name: &str) -> Texture {
        let view = &self.views[name];
        Texture {
            texture: &self.textures[view.id],
            origin: &view.origin,
            dimensions: &view.dimensions,
        }
    }
}

fn main()
{
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // An example texture.
    let texture = TextureSource {
        texture_file: std::path::PathBuf::from("assets/opengl.png"),
        texture_format: image::PNG,
        slices: vec![
            SliceSource { name: "badger".to_owned(), origin: cgmath::vec2(0, 0), dimensions: cgmath::vec2(600, 297) },
        ]
    };

    // Load all our textures.
    let sources = vec![texture];
    let collection = TextureCollection::new(&display, sources.into_iter());

    // Access the badger texture.
    let badger_tex = collection.texture("badger");
    println!("{:?}", badger_tex);
}

