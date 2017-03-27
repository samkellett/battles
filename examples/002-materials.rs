// The material collection owns all materials and provides const references
// to them to other components in the engine.
// It is constant and so is given all of the materials on construction and
// cannot remove or add materials during the main program execution.
// A material is defined as:
//  - A shader program: glium::Program,
//  - A texture: glium::texture::Texture2d.
// Materials will be accessible by a String key, logarithmic complexity is
// fine here as the returned reference should be saved and the key not given
// after the start of the program.
// The constructor should take an iterator of a trait that defines how to
// access the three underlying types. This trait will provide shader sources
// as references to strings and a path to a png file for the texture. Extra
// texture formats can be added later. This should be a trait instead of
// a concrete type so that it is possible to load this information from
// various sources such as a function (ie. for testing) or a configuration
// file (ie, in production).

extern crate glium;
extern crate image;

use std::collections::HashMap;

struct Material {
    pub program: glium::Program,
    pub texture: glium::texture::Texture2d,
}

struct MaterialSource {
    // String key used to access this material.
    pub name: String,
    // Access the vertex shader source code.
    pub vertex_shader: String,
    // Access the fragment shader source code.
    pub fragment_shader: String,
    // Access the path to the texture file.
    pub texture_file: std::path::PathBuf,
    // The image type of the texture.
    pub texture_format: image::ImageFormat,
}

struct MaterialCollection {
    materials: HashMap<String, Material>,
}

impl MaterialCollection {
    fn new<T>(display: &T, sources: &[MaterialSource]) -> MaterialCollection
        where T: glium::backend::Facade
    {
        let mut materials = HashMap::new();
        for source in sources.iter() {
            let program = glium::Program::from_source(display,
                                                      &source.vertex_shader,
                                                      &source.fragment_shader,
                                                      None)
                .unwrap();

            let texture = {
                let file = std::fs::File::open(&source.texture_file).unwrap();
                let file = std::io::BufReader::new(file);

                let image = image::load(file, source.texture_format)
                    .unwrap()
                    .to_rgba();

                let image_dimensions = image.dimensions();
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

                glium::texture::Texture2d::new(display, image).unwrap()
            };

            let material = Material { program: program, texture: texture };
            materials.insert(source.name.clone(), material);
        }

        MaterialCollection { materials: materials }
    }
}

fn main() {
    use glium::DisplayBuild;
    use std::path::PathBuf;

    let v = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let f = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let sources = vec![MaterialSource {
                           name: "badger".to_owned(),
                           vertex_shader: v.to_owned(),
                           fragment_shader: f.to_owned(),
                           texture_file: PathBuf::from("assets/opengl.png"),
                           texture_format: image::PNG,
                       }];
    let collection = MaterialCollection::new(&display, &sources);
}
