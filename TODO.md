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
    let config = Config::from_file("assets/game.toml")

    // Initialise
    let game_objects = GameObjects::from_config(&config);
        // ...internally... a vector of GameObjects.

    // Initialse engines.
    let renderer = RenderEngine::from_config(&config, &game_objects);
        // ...internally...
        let textures = ...
        let materials = ...

        //  ...makes a vector of references to renderable components.

    loop {
        renderer.draw()
    }
}

