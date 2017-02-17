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
// access the three underlying types. This trait will provide std::path::Path
// objects for each shader and the texture. This should be a trait instead of
// a concrete type so that it is possible to load this information from
// various sources such as a function (ie. for testing) or a configuration
// file (ie, in production).

struct MaterialCollection;

fn main()
{
}

