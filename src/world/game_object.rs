use render::sprites::{Sprite, SpriteId};
use render::transform::Transform;

// on-disk representation.
#[derive(Debug, Deserialize)]
pub struct GameObjectSource {
    pub position: [i32; 2],
    pub sprite: String,
}

pub struct GameObject {
    pub transform: Transform,
    pub sprite: SpriteId,
}

impl GameObject {
    pub fn new<F>(source: GameObjectSource, get_sprite_id: F) -> GameObject
        where F: Fn(&str) -> SpriteId
    {
        let mut transform = Transform::new();
        transform.translate(source.position[0] as f32, source.position[1] as f32, 0.0);

        GameObject {
            transform: transform,
            sprite: get_sprite_id(&source.sprite),
        }
    }

    pub fn from_config<I, F>(sources: I, get_sprite_id: &F) -> Vec<GameObject>
        where F: Fn(&str) -> SpriteId,
              I: Iterator<Item = GameObjectSource>
    {
        sources.map(|s| GameObject::new(s, get_sprite_id)).collect::<Vec<_>>()
    }
}
