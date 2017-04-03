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

// The internal representation of a material.
#[derive(Debug)]
struct Material {
    pub program: glium::Program,
    pub texture: glium::texture::Texture2d,
}

// A trait that provides a representation of a material.
trait MaterialSource {
    // Identifier key used to access this material.
    fn name(&self) -> &str;
    // Access the vertex shader source code.
    fn vertex_shader(&self) -> &str;
    // Access the fragment shader source code.
    fn fragment_shader(&self) -> &str;
    // Access the path to the texture file.
    fn texture_file(&self) -> &std::path::Path;
    // The image type of the texture.
    fn texture_format(&self) -> image::ImageFormat;
}

// A non-owning implementation of the MaterialSource trait.
struct MaterialView<'a> {
    pub name: &'a str,
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub texture_file: &'a std::path::Path,
    pub texture_format: image::ImageFormat,
}

impl<'a> MaterialSource for MaterialView<'a> {
    fn name(&self) -> &str { self.name }
    fn vertex_shader(&self) -> &str { self.vertex_shader }
    fn fragment_shader(&self) -> &str { self.fragment_shader }
    fn texture_file(&self) -> &std::path::Path { self.texture_file }
    fn texture_format(&self) -> image::ImageFormat { self.texture_format }
}

// A collection of materials.
struct MaterialCollection {
    materials: HashMap<String, Material>,
}

impl MaterialCollection {
    // Create a new material collection from an iterator of MaterialSource's.
    fn new<D, I>(display: &D, sources: I) -> MaterialCollection
        where D: glium::backend::Facade,
              I: Iterator,
              I::Item: MaterialSource,
    {
        let mut materials = HashMap::new();
        for source in sources {
            // Build the shader program.
            let program = glium::Program::from_source(display,
                                                      source.vertex_shader(),
                                                      source.fragment_shader(),
                                                      None)
                .unwrap();

            // Build the texture.
            let texture = {
                let file = std::fs::File::open(source.texture_file()).unwrap();
                let file = std::io::BufReader::new(file);

                let image = image::load(file, source.texture_format())
                    .unwrap()
                    .to_rgba();

                let image_dimensions = image.dimensions();
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                               image_dimensions);

                glium::texture::Texture2d::new(display, image).unwrap()
            };

            let material = Material {
                program: program,
                texture: texture,
            };
            materials.insert(source.name().to_owned(), material);
        }

        MaterialCollection { materials: materials }
    }

    // Get a reference to a registered material.
    fn material(&self, name: &str) -> &Material {
        &self.materials[name]
    }
}

fn main() {
    use glium::DisplayBuild;

    use std::path::Path;

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // Example shaders.
    let v = include_str!("../assets/simple.vert");
    let f = include_str!("../assets/simple.frag");

    // An example material.
    let material = MaterialView {
        name: "badger",
        vertex_shader: v,
        fragment_shader: f,
        texture_file: Path::new("assets/opengl.png"),
        texture_format: image::PNG,
    };

    // Load all our materials.
    let sources = vec![material];
    let collection = MaterialCollection::new(&display, sources.into_iter());

    // Access the badger material.
    let badger_mat = collection.material("badger");
    println!("{:?}", badger_mat);
}
