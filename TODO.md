# v0.1.0 Roadmap:
- [x] Be able to draw a square with a texture.
- [x] Be able to load materials from disk (shader program and texture).
- [ ] Be able to draw a map of squares.


enum Components {
    Tranform(f32, f32, f32),
    Renderable(Sprite, Mesh),
}

struct GameObject {
    Vec<Components> components;
}

fn main()
{
    let updater = UpdateEngine::new(...)
    let renderer = RenderEngine::new(...)

    let textures = ...
    let materials = ...

    let game_objects = ...

    updater.load(game_objects)
    renderer.load(game_objects)

    loop {
        updater.update()
        renderer.draw()
    }
}

