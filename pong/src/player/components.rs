use bevy::prelude::*;

use super::systems::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(player: Player, x: f32, y: f32) -> Self {
        let sprite = Sprite {
            color: Color::WHITE,
            rect: Some(Rect::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        };

        PlayerBundle {
            player,
            sprite_bundle: SpriteBundle {
                sprite,
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
        }
    }
}
