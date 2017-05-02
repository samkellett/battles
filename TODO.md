# v0.1.0 Roadmap:
- [x] Be able to draw a square with a texture.
- [x] Be able to load materials from disk (shader program and texture).
- [ ] Be able to draw a map of squares.

enum Renderable {
    Sprite(...),
    Text(...),
}

struct Tranform(f32, f32, f32)

struct GameObject {
    Transform transform;
    SpriteIndex sprite;
}

fn main()
{
    let config = Config::from_file("assets/game.toml")

    // Initialise render engine (and load all sprites)
    let renderer = RenderEngine::from_config(&config);
        // ...internally...
        let textures = ...
        let materials = ...

    // ...
    let updater = LuaEngine::new();

    // Load all game objects, gets sprite index from renderer.
    let game_objects = GameObjects::from_config(&config, &renderer);
        // ...internally... a vector of GameObjects.

    let executor = SerialExecutor::new();

    loop {
        for game_object in game_objects {
            game_object.update(&updater);
            game_object.draw(&renderer);
        }

        executor.execute(&updater, &renderer);
    }
}

